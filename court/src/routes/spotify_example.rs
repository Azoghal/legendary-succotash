use rocket::serde::json::{json, Value};

use crate::services::spotify_example;

#[get("/artist-popularity/<id>")]
pub fn get_artist_popularity(id: &str) -> Value {
    json!(spotify_example::get_artist_popularity())
}
