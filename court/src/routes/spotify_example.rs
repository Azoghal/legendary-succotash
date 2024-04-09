use rocket::{serde::json::Json, State};

use crate::{
    errors,
    models::spotify::Popularity,
    services::{spotify::UserSpotifyApi, spotify_example},
    spotify::SpotifyApi,
};

#[get("/artist-popularity/<id>")]
pub async fn get_artist_popularity(
    id: &str,
    spotify: &State<SpotifyApi>,
) -> Result<Json<Popularity>, errors::Error> {
    let res = spotify_example::get_artist_popularity(id, spotify).await?;
    Ok(Json(res))
}

#[get("/authorize_url")]
pub async fn get_client_url(
    spotify: &State<UserSpotifyApi>,
) -> Result<Json<String>, errors::Error> {
    let res = spotify_example::get_client_url(spotify).await?;
    Ok(Json(res))
}
