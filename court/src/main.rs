use rocket::fs::{relative, FileServer, NamedFile};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::{tokio, State};

use rspotify::{ClientCredsSpotify, Credentials};

use std::env::VarError;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;

mod routes;
mod services;

pub mod errors;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests;

// fallback to serve index.html. This is hit for anything not in /assets and not a different rust route
#[get("/<_..>", rank = 101)]
async fn fallback() -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!(
        "{}/../lodge/dist/index.html",
        env!("CARGO_MANIFEST_DIR")
    )))
    .await
    .ok()
}

struct SpotifyApi {
    client: ClientCredsSpotify,
}

impl SpotifyApi {
    // use macro to make blocking so we can just call it easily on server startup.
    #[rocket::tokio::main]
    async fn new() -> Self {
        let creds = Credentials::from_env().expect("failed to get credentials from env");
        let client = ClientCredsSpotify::new(creds);

        client
            .request_token()
            .await
            .expect("failed to get spotify token");

        SpotifyApi { client }
    }
}

#[launch]
fn rocket() -> _ {
    // TODO come back and fix the cors rules
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();
    let spotify = SpotifyApi::new();
    let auth0 = routes::auth0::Auth0::from_env().unwrap();

    rocket::build()
        .manage(spotify)
        .manage(auth0) // TODO we actually need to not do this because secrets
        .mount(
            "/api/v1",
            routes![
                routes::spotify_example::get_artist_popularity,
                routes::session::session_user,
                routes::session::session_user_fail
            ],
        )
        .mount(
            "/assets",
            FileServer::from(relative!("../lodge/dist/assets")).rank(1), // this replaces /<file..> route
        )
        .mount(
            "/",
            routes![
                fallback,
                routes::auth0::auth0_redirect,
                routes::auth0::auth0_callback,
                routes::auth0::logged_in
            ],
        )
        .attach(cors)
}
