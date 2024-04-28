use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use crypto_hash::hex_digest;
use jsonwebtoken::{
    decode, decode_header, jwk, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation,
};
use rand::Rng;
use std::env;

use crate::models::{session::NewSession, users::NewUser};
use crate::services::{auth0, users};
use crate::{errors, services, SuccDb};

pub fn random_state_string() -> String {
    use rand::{distributions::Alphanumeric, thread_rng};
    let mut rng = thread_rng();
    let chars: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    chars
}

#[get("/login")]
pub fn auth0_redirect(
    cookies: &CookieJar,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    let state = random_state_string();
    let my_cookie = Cookie::build(("state", state.clone())).same_site(SameSite::Lax); // might be able to get rid of samesite lax if hosted properly
    cookies.add(my_cookie);
    let uri = format!("https://{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile&state={}",
        settings.auth0_tenant_domain,
        settings.client_id,
        &settings.redirect_uri,
        state);
    Ok(response::Redirect::to(uri))
}

// The callback we get from Auth0
// contains code and state.
// state should be same as we sent
#[get("/auth/callback?<code>&<state>")]
pub async fn auth0_callback(
    db: SuccDb,
    code: String,
    state: String,
    cookies: &CookieJar<'_>,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    let cook = cookies.get_pending("state");
    if let Some(cookie) = cook {
        if state != cookie.value() {
            return Err(rocket::http::Status::Forbidden);
        }
    } else {
        error!("state cookie bad");
        return Err(rocket::http::Status::BadRequest);
    }
    cookies.remove("state");

    let token_request = settings.create_token_request(&code);
    let endpoint = format!("https://{}/oauth/token", settings.auth0_tenant_domain);
    let client = reqwest::Client::new();

    let Ok(resp_json) = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .json(&token_request)
        .send()
        .await
    else {
        error!("failed to send token request");
        return Err(rocket::http::Status::BadRequest);
    };

    let resp = match resp_json.json::<TokenResponse>().await {
        Ok(r) => r,
        Err(e) => {
            error!("failed to parse TokenResponse json: {e:?}");
            return Err(rocket::http::Status::InternalServerError);
        }
    };
    let jwt = resp.id_token;

    let Ok(claims) = decode_jwt(&jwt, settings).await else {
        error!("failed to decode token");
        return Err(rocket::http::Status::BadRequest);
    };

    let user = NewUser {
        auth0subject: claims.sub,
        name: claims.nickname,
    };

    let Ok(user) = users::get_or_create_user(&db, user).await else {
        error!("failed to get or create user");
        return Err(rocket::http::Status::InternalServerError);
    };

    let jwt_hash = hex_digest(crypto_hash::Algorithm::SHA256, jwt.clone().as_bytes());

    let new_session = NewSession {
        user_id: user.id,
        expires: claims.exp as i32,
        jwt_hash: jwt_hash.clone(),
        jwt,
    };

    let Ok(_) = services::auth0::create_session(&db, new_session).await else {
        error!("failed to create session");
        return Err(rocket::http::Status::InternalServerError);
    };

    let cookie = Cookie::build(("session", jwt_hash))
        .same_site(SameSite::Lax)
        .path("/")
        .secure(true)
        .http_only(true);
    cookies.add(cookie);

    Ok(response::Redirect::to("/loggedin"))
}

#[get("/loggedin")]
pub fn logged_in() -> response::Redirect {
    info!("logged in!");
    response::Redirect::to("/")
}

pub struct Auth0 {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth0_tenant_domain: String,
}

impl Auth0 {
    pub fn from_env() -> Result<Auth0, errors::Error> {
        let app_settings = Auth0 {
            client_id: env::var("AUTH0_CLIENT_ID")?,
            client_secret: env::var("AUTH0_CLIENT_SECRET")?,
            redirect_uri: env::var("AUTH0_REDIRECT_URI")?,
            auth0_tenant_domain: env::var("AUTH0_DOMAIN")?,
        };
        Ok(app_settings)
    }

    // make mame a token request with our secret and the code recieved from auth0
    fn create_token_request(&self, code: &str) -> TokenRequest {
        TokenRequest {
            grant_type: String::from("authorization_code"),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: code.to_string(),
            redirect_uri: self.redirect_uri.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct TokenResponse {
    access_token: String,
    expires_in: u32,
    id_token: String,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct IdTokenClaims {
    sub: String,
    nickname: String,
    exp: usize,
}

async fn get_jwks(settings: &State<Auth0>) -> Result<jwk::JwkSet, errors::Error> {
    let endpoint = format!(
        "https://{}/.well-known/jwks.json",
        settings.auth0_tenant_domain
    );
    let client = reqwest::Client::new();

    let resp_json = match client.get(endpoint).send().await {
        Ok(resp_json) => resp_json,
        Err(e) => {
            error!("failed to send jwks request {e}");
            return Err(e.into());
        }
    };

    let jwks: jwk::JwkSet = resp_json.json::<jwk::JwkSet>().await?;

    Ok(jwks)
}

async fn decode_jwt(jwt: &str, settings: &State<Auth0>) -> Result<IdTokenClaims, errors::Error> {
    let header = decode_header(jwt)?;
    let kid = match header.kid {
        Some(k) => {
            info!("kid from header: {k}");
            k
        }
        None => return Err(errors::Error::NotFound("no kid in jwt".into())),
    };

    let jwks = get_jwks(settings).await?;

    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();

                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_audience(&[&settings.client_id]);
                validation.validate_exp = false;
                let decoded_token = decode::<IdTokenClaims>(jwt, &decoding_key, &validation);
                println!("{:?}", decoded_token);
                let token_claims = match decoded_token {
                    Ok(t) => t,
                    Err(e) => return Err(e.into()),
                };
                Ok(token_claims.claims)
            }
            _ => Err(errors::Error::UnexpectedAlg(
                "jwks algorithm not RSA".into(),
            )),
        }
    } else {
        Err(errors::Error::NotFound(
            "No matching JWK found for the given kid".into(),
        ))
    }
}

// Session user is the struct used for request guards for authenticated endpoints
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SessionUser {
    pub user_sub: String,
    pub name: String,
    pub id: i32,
}

// To generate a session user for authenticated routes, we lookup the hashed jwt
// and fetch user details for relevant session from db
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for SessionUser {
    type Error = ();
    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<SessionUser, ()> {
        use rocket::http::Status;
        use rocket::request::Outcome;

        let Some(session_cookie) = request.cookies().get("session") else {
            info!("no session cookie present");
            return Outcome::Forward(Status::Unauthorized);
        };
        let Ok(session_id) = session_cookie.value().parse::<String>() else {
            error!("failed to parse cookie value");
            return Outcome::Forward(Status::InternalServerError);
        };

        let db = match SuccDb::from_request(request).await {
            Outcome::Success(db) => db,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(s) => return Outcome::Forward(s),
        };

        // TODO replace this with a nicer single db query usinga join, once migrated over to using actual SQL
        let Ok(Some(session)) = auth0::get_session_by_hash(&db, session_id).await else {
            error!("failed fetch session from db");
            return Outcome::Forward(Status::InternalServerError);
        };

        let Ok(session_user) = users::get_user(&db, session.user_id).await else {
            error!("failed fetch session user from db");
            return Outcome::Forward(Status::InternalServerError);
        };

        let user = SessionUser {
            user_sub: session_user.auth0subject,
            name: session_user.name,
            id: session_user.id,
        };
        Outcome::Success(user)
    }
}
