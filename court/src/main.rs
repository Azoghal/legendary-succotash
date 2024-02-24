use rocket::fs::NamedFile;
use rocket::tokio;
use rspotify::{ClientCredsSpotify, Credentials};

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

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(&format!("{}/../lodge/dist", env!("CARGO_MANIFEST_DIR"))).join(file);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
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

    rocket::build()
        .manage(spotify)
        .mount("/", rocket::routes![files])
        .mount(
            "/api/v1",
            routes![
                routes::succotash::get_recipes,
                routes::spotify_example::get_artist_popularity
            ],
        )
        .attach(cors)
}
