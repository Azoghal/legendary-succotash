use rocket::{serde::json::Json, State};

use crate::{models::Popularity, services::spotify_example, SpotifyApi};

#[get("/artist-popularity/<id>")]
pub async fn get_artist_popularity(
    id: &str,
    spotify: &State<SpotifyApi>,
) -> Result<Json<Popularity>, ()> {
    let res = spotify_example::get_artist_popularity(id, spotify).await?;
    Ok(Json(res))
}
