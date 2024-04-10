use rocket::{http::Status, response, serde::json::Json, State};

use crate::{
    errors,
    models::spotify::AuthUrl,
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
    Ok(Json(Popularity { popularity: res }))
}

#[get("/authorize_url")]
pub async fn get_client_url(
    spotify: &State<UserSpotifyApi>,
) -> Result<Json<AuthUrl>, errors::Error> {
    let res = spotify_example::get_client_url(spotify).await?;
    Ok(Json(AuthUrl { url: res }))
}

#[get("/sp/callback")]
pub async fn sp_callback() -> Result<response::Redirect, Status> {
    info!("you successfully hit sp callback");
    Ok(response::Redirect::to("/"))
}
