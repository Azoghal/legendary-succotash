use std::env::VarError;

use rand::Rng;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

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

    let resp = resp_json.json::<TokenResponse>().await;

    match resp {
        Ok(r) => {
            info!("it worked {:?}", r)
        }
        Err(e) => {
            error!("failed to deserialize token response {:?}", e);
            return Err(rocket::http::Status::BadRequest);
        }
    }

    // // TODO: Can we unwrap here because we know for certain we've populated the cert in the db?
    // let pub_key: Vec<u8> = db.get(b"jwt_pub_key_pem").unwrap().unwrap().to_vec();
    // let payload = decode_and_validate_jwt(
    //     pub_key,
    //     &resp.id_token,
    //     &settings.client_id,
    //     &settings.auth0_domain,
    // )
    // .map_err(|_| Status::Unauthorized)?;
    // let user = get_or_create_user(&db, &payload).map_err(|e| match e.downcast_ref() {
    //     Some(AuthError::MalformedJWT { .. }) => Status::BadRequest,
    //     _ => Status::InternalServerError,
    // })?;

    // let jwt = &resp.id_token.clone();
    // let hashed_jwt = hex_digest(HashAlgorithm::SHA256, jwt.as_bytes());
    // let new_session = Session {
    //     user_id: user.user_id,
    //     expires: payload.exp,
    //     raw_jwt: jwt.as_bytes().to_vec(),
    // };
    // let encoded_session = serialize(&new_session).map_err(|_| Status::Unauthorized)?;
    // let session_key = make_key!("sessions/", hashed_jwt.clone());
    // db.set(session_key.0, encoded_session).unwrap();
    // let cookie = Cookie::build("session", hashed_jwt)
    //     .path("/")
    //     .secure(true)
    //     .http_only(true)
    //     .finish();
    // cookies.add(cookie);

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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Auth0JWTPayload {
    email: String,
    user_id: String,
    exp: i64,
    iss: String,
    aud: String,
}
