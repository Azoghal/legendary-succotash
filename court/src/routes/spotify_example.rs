use rocket::{http::Status, response, serde::json::Json, State};

use crate::{
    errors,
    models::spotify::AuthUrl,
    models::spotify::Popularity,
    services::{spotify::UserSpotifyApi, spotify_example},
    spotify::SpotifyApi,
};

use super::auth0::SessionUser;

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

#[get("/sp/callback?<code>")]
pub async fn sp_callback(
    code: String,
    user: SessionUser,
    spotify: &State<UserSpotifyApi>,
) -> Result<response::Redirect, errors::Error> {
    info!("you successfully hit spotify callback with a user! We can now associate these!");
    info!("Now we know {} can sign into a spotify account", user.name);
    spotify.get_the_token(&code).await?;
    spotify.do_something_interesting().await?;
    Ok(response::Redirect::to("/notlanding"))
}

#[get("/sp/callback", rank = 2)]
pub async fn sp_callback_no_user() -> Result<response::Redirect, Status> {
    info!("you successfully hit sp callback, but you didn't have a user session");
    Ok(response::Redirect::to("/"))
}
