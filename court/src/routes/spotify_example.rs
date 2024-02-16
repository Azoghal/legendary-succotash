use rocket::serde::json::{json, Value};

use crate::services::spotify_example;

#[get("/artist-name")]
pub fn get_artist_name() -> Value {
    json!(spotify_example::get_artist_name())
}
