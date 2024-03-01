use std::env::VarError;

use rand::distributions::DistString;
use rand::Rng;
use rocket::http::{uri, Cookie, CookieJar, SameSite, Status};
use rocket::response;
use rocket::State;

/// Helper to create a random string 30 chars long.
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
    let my_cookie = Cookie::build(("state", state.clone())).same_site(SameSite::Lax);
    cookies.add(my_cookie); // try samesite lax because of 127 vs localhost?
    let uri = format!("https://{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile&state={}",
        settings.auth0_domain,
        settings.client_id,
        &settings.redirect_uri,
        state);
    Ok(response::Redirect::to(uri))
}

// The callback we get from Auth0
// contains code and state.
// state should be same as we sent
#[get("/callback?<code>&<state>")]
pub fn auth0_callback(
    code: String,
    state: String,
    cookies: &CookieJar,
    // db: State<DB>,
    // settings: &State<Auth0>,
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

    // let tr = settings.token_request(&code);

    // // TODO: The call to /oauth/token can panic if there are any misconfigurations: The wrong
    // // secret, for instance; also, if the user is unauthorized. We need a nicer way to handle
    // // unauthorized here. Also, we need a nicer way to debug the response. We deserialize directly
    // // into a TokenResponse, but the auth0 api will return this in the event of misconfiguration:
    // //   "{\"error\":\"access_denied\",\"error_description\":\"Unauthorized\"}"
    // let token_endpoint = format!("https://{}/oauth/token", settings.auth0_domain);
    // println!("token endpoint time: {:?}", token_endpoint);
    // let client = reqwest::Client::new();
    // let resp: TokenResponse = client
    //     .post(&token_endpoint)
    //     .header("Content-Type", "application/json")
    //     .body(to_vec(&tr).unwrap())
    //     .send()
    //     .unwrap()
    //     .json()
    //     .expect("could not deserialize response from /oauth/token");

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
    auth0_domain: String,
}

impl Auth0 {
    // TODO replace VarError with real error
    pub fn from_env() -> Result<Auth0, VarError> {
        let app_settings = Auth0 {
            client_id: std::env::var("AUTH0_CLIENT_ID")?,
            client_secret: std::env::var("AUTH0_CLIENT_SECRET")?,
            redirect_uri: std::env::var("AUTH0_REDIRECT_URI")?,
            auth0_domain: std::env::var("AUTH0_DOMAIN")?,
        };
        Ok(app_settings)
    }
}
