use rocket::{http::Status, response, serde::json::Json};

use crate::{errors, models::spotify::AuthUrl, services::spotify::UserSpotifyHelper, SuccDb};

use super::auth0::SessionUser;

#[get("/authorize_url")]
pub async fn get_client_url(
    spotify_helper: UserSpotifyHelper,
) -> Result<Json<AuthUrl>, errors::Error> {
    let res = spotify_helper.get_spotify_auth_url().await?;
    Ok(Json(AuthUrl { url: res }))
}

#[get("/sp/callback?<code>")]
pub async fn sp_callback(
    db: SuccDb,
    code: String,
    user: SessionUser,
    spotify_helper: UserSpotifyHelper,
) -> Result<response::Redirect, errors::Error> {
    spotify_helper
        .get_new_user_token(&db, user.id, &code)
        .await?;
    Ok(response::Redirect::to("/notlanding"))
}

#[get("/sp/callback", rank = 2)]
pub async fn sp_callback_no_user() -> Result<response::Redirect, Status> {
    info!("no user session");
    Ok(response::Redirect::to("/"))
}
