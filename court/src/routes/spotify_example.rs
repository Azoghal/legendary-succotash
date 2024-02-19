use rocket::serde::json::Json;

use crate::{models::Popularity, services::spotify_example};

#[get("/artist-popularity/<id>")]
pub async fn get_artist_popularity(id: &str) -> Result<Json<Popularity>, ()> {
    let res = spotify_example::get_artist_popularity(id).await?;
    Ok(Json(res))
}
