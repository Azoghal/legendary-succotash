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

// fallback from /<file..> - if a static file is not found, then serve the template for frontend
#[get("/<_..>", rank = 101)]
async fn fallback() -> Option<NamedFile> {
    // let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // let path_segment = format!("{}/../lodge/dist/index.html", manifest_dir);
    // let path = Path::new(&path_segment);
    // info!("redirecting to frontend for route {:?}", file);

    NamedFile::open(Path::new(&format!(
        "{}/../lodge/dist/index.html",
        env!("CARGO_MANIFEST_DIR")
    )))
    .await
    .ok()
}

// High rank so that e.g. fallthrough from request guards does not hit this
// #[get("/<file..>", rank = 100)]
// async fn files(file: PathBuf) -> Option<NamedFile> {
//     let mut path = Path::new(&format!("{}/../lodge/dist", env!("CARGO_MANIFEST_DIR"))).join(file);
//     if path.is_dir() {
//         path.push("index.html");
//     }

//     NamedFile::open(path).await.ok()
// }

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
                routes::succotash::get_recipes,
                routes::spotify_example::get_artist_popularity,
                routes::spotify_example::user_session_test,
                routes::spotify_example::user_session_test_fail
            ],
        )
        .mount(
            "/assets",
            FileServer::from(relative!("../lodge/dist/assets")).rank(1),
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
