use rocket::{http::Status, response, serde::json::Json, State};

use crate::{
    errors,
    models::spotify::{AuthUrl, CurrentPlaying, Popularity},
    services::{
        spotify::{UserSpotifyApi, UserSpotifyHelper},
        spotify_example,
    },
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

#[get("/authorize_url")]
pub async fn get_client_url(
    spotify_helper: UserSpotifyHelper,
) -> Result<Json<AuthUrl>, errors::Error> {
    let res = spotify_helper.get_spotify_auth_url().await?;
    Ok(Json(AuthUrl { url: res }))
}

// TODO remove
#[get("/temp_get_access_token")]
pub async fn temp_get_access_token(
    user: SessionUser,
) -> Result<Json<CurrentPlaying>, errors::Error> {
    let Ok(res) = UserSpotifyApi::load_user_token(user.id).await else {
        return Err(errors::Error::NotFound("Oh dear".into()));
    };

    Ok(Json(CurrentPlaying {
        title: format!("Your code expires at {:?}", res.expires_at),
    }))
}

#[get("/sp/callback?<code>")]
pub async fn sp_callback(
    code: String,
    user: SessionUser,
    spotify_helper: UserSpotifyHelper,
) -> Result<response::Redirect, errors::Error> {
    info!("you successfully hit spotify callback with a user! We can now associate these!");
    info!("Now we know {} can sign into a spotify account", user.name);
    spotify_helper.get_new_user_token(user.id, &code).await?;
    Ok(response::Redirect::to("/notlanding"))
}

#[get("/sp/callback", rank = 2)]
pub async fn sp_callback_no_user() -> Result<response::Redirect, Status> {
    info!("you successfully hit sp callback, but you didn't have a user session");
    Ok(response::Redirect::to("/"))
}
