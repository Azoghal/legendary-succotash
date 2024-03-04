use std::env::VarError; // TODO remove

use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use crypto_hash::hex_digest;
use jsonwebtoken::{
    decode, decode_header, jwk, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation,
};
use rand::Rng;

use crate::errors;
use crate::models::NewUser;
use crate::services::users;

pub fn random_state_string() -> String {
    use rand::{distributions::Alphanumeric, thread_rng};
    let mut rng = thread_rng();
    let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
    chars
}

#[get("/login")]
pub fn auth0_redirect(
    cookies: &CookieJar,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    // TODO make an actual random string
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
#[get("/callback?<code>&<state>")]
pub async fn auth0_callback(
    code: String,
    state: String,
    cookies: &CookieJar<'_>,
    // db: State<DB>,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    let cook = cookies.get_pending("state");
    info!("{:?}", cook);
    if let Some(cookie) = cook {
        if state != cookie.value() {
            return Err(rocket::http::Status::Forbidden);
        }
    } else {
        println!("cookie state bad");
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

    info!("resp_json {:?}", resp_json);

    let resp = match resp_json.json::<TokenResponse>().await {
        Ok(r) => r,
        Err(e) => {
            error!("failed to parse TokenResponse json");
            return Err(rocket::http::Status::InternalServerError);
        }
    };
    let jwt = resp.id_token;

    let Ok(claims) = decode_jwt(&jwt, settings).await else {
        error!("failed to decode token");
        return Err(rocket::http::Status::BadRequest);
    };

    let user = NewUser {
        auth0subject: &claims.sub,
        name: &claims.nickname,
    };

    let Ok(user) = users::get_or_create_user(user) else {
        return Err(rocket::http::Status::InternalServerError);
    };

    info!("the user that logged in: {:?}", user);

    // let hashed_jwt = hex_digest(crypto_hash::Algorithm::SHA256, jwt.clone().as_bytes());
    let new_session = Session {
        user_id: user.id,
        expires: claims.exp,
        raw_jwt: jwt.clone().as_bytes().to_vec(),
    };

    // TODO work out how we want to do sessions
    // the cookie value should probably be hashed jwt
    // and we can whack the session into the db in some place

    // for now we'll just whack what we want in plain text. Secure :)
    let cookie = Cookie::build(("session", user.clone().auth0subject))
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
    // TODO replace VarError with real error
    pub fn from_env() -> Result<Auth0, VarError> {
        let app_settings = Auth0 {
            client_id: std::env::var("AUTH0_CLIENT_ID")?,
            client_secret: std::env::var("AUTH0_CLIENT_SECRET")?,
            redirect_uri: std::env::var("AUTH0_REDIRECT_URI")?,
            auth0_tenant_domain: std::env::var("AUTH0_DOMAIN")?,
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

    let Ok(resp_json) = client.get(endpoint).send().await else {
        error!("failed to send jwks request");
        return Err(errors::Error::Placeholder(
            "failed to send jks request".into(),
        ));
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
        None => return Err(errors::Error::Placeholder("no kid in jwt".into())),
    };

    let jwks = get_jwks(settings).await?;

    info!("the jwks: {:?}", jwks);

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
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        Err(errors::Error::Placeholder(
            "No matching JWK found for the given kid".into(),
        ))
    }
}

// we whack a session in a cookie, that we can then grab on the frontend
struct Session {
    user_id: i32,
    expires: usize,
    raw_jwt: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SessionUser {
    pub user_sub: String,
    pub name: String,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for SessionUser {
    type Error = ();
    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<SessionUser, ()> {
        let session_id: Option<String> = request
            .cookies()
            .get("session")
            .and_then(|cookie| cookie.value().parse().ok());
        match session_id {
            None => {
                println!("no session id");
                rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized)
            }
            Some(session_sub) => {
                println!("session id (the auth0subject to lookup): {}", session_sub);
                // TODO Now get a db connection and lookup to populate the session boy.
                let user = SessionUser {
                    user_sub: session_sub,
                    name: "NotRealName".into(),
                };
                rocket::outcome::Outcome::Success(user)
            }
        }
    }
}
