use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub mod auth0;
pub mod spotify;
pub mod spotify_example;
pub mod spotify_tokens;
pub mod users;

// TODO we want to move this towards being a fairing or something?
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
