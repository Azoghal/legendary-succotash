use dotenvy::dotenv;
use rocket::{
    fs::{relative, FileServer, NamedFile},
    serde::json::Json,
};
use rocket_sync_db_pools::database;
use services::spotify;
use std::path::Path;

use court::ts_client;

#[macro_use]
extern crate rocket;
extern crate diesel;

mod routes;
mod services;

pub mod errors;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests;

#[database("db")]
pub struct SuccDb(diesel::PgConnection);

// TODO consider whacking the whole frontend under some route, because at the moment we 200ok and serve the landing page for even completely wrong requests
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

#[get("/", rank = 100)]
async fn api_fallback() -> Json<()> {
    error!("api route does not exist");
    Json(())
}

#[ts_client(bill)]
fn mock_boy() {}

// #[get("/well-i-knever")]
// async fn well_no_call(db: SuccDb) {
//     db.run(|c| c.insert)
// }

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    // TODO come back and fix the cors rules
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();
    let spotify = spotify::SpotifyApi::new();
    let auth0 = routes::auth0::Auth0::from_env().unwrap();

    rocket::build()
        .attach(SuccDb::fairing())
        .manage(spotify)
        .manage(auth0) // I think for now, that this is fine...
        .mount(
            "/api/v1",
            routes![
                api_fallback,
                routes::spotify_auth::get_client_url,
                routes::spotify_example::get_artist_popularity,
                routes::spotify_example::get_current_playing,
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
                routes::auth0::logged_in,
                routes::spotify_auth::sp_callback,
                routes::spotify_auth::sp_callback_no_user
            ],
        )
        .attach(cors)
}
