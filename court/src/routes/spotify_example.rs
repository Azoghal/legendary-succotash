use rocket::{serde::json::Json, State};

use crate::{
    errors,
    models::spotify::{CurrentPlaying, Popularity},
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

#[get("/user/currently_playing")]
pub async fn get_current_playing(
    spotify: UserSpotifyApi,
) -> Result<Json<CurrentPlaying>, errors::Error> {
    let res = spotify.get_current_playing().await;
    match res {
        Ok(r) => {
            let title = match r {
                Some(s) => s,
                None => "nothing playing".into(),
            };

            Ok(Json(CurrentPlaying { title }))
        }
        Err(e) => {
            error!("failed to get current playing: {}", e);
            Err(e)
        }
    }
}
