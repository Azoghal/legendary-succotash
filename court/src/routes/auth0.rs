use std::env::VarError;

use rand::distributions::DistString;
use rand::Rng;
use rocket::http::{uri, Cookie, CookieJar, Status};
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
    // cookies: &CookieJar,
    settings: &State<Auth0>,
) -> Result<response::Redirect, Status> {
    // TODO make an actual random string
    let state = random_state_string();
    // cookies.add(Cookie::new("state", state.clone()));
    let uri = format!("https://{}/authorize?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile&state={}",
        settings.auth0_domain,
        settings.client_id,
        &settings.redirect_uri,
        state);
    Ok(response::Redirect::to(uri))
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
