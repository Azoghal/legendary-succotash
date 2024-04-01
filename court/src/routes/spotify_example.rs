use rocket::{serde::json::Json, State};

use super::auth0::SessionUser;
use crate::{
    errors,
    models::{Popularity, User},
    services::{spotify_example, users::get_user_by_auth0_subject},
    SpotifyApi,
};

#[get("/artist-popularity/<id>")]
pub async fn get_artist_popularity(
    id: &str,
    spotify: &State<SpotifyApi>,
) -> Result<Json<Popularity>, errors::Error> {
    let res = spotify_example::get_artist_popularity(id, spotify).await?;
    Ok(Json(res))
}

// TODO move these to own set of routes
#[get("/user-session-test")]
pub async fn user_session_test(user: SessionUser) -> Result<Json<Option<User>>, errors::Error> {
    info!(
        "If you see this, then the request guard worked! {}",
        user.user_sub
    );
    let user = get_user_by_auth0_subject(&user.user_sub)?;
    Ok(Json(user))
}

#[get("/user-session-test", rank = 2)]
pub async fn user_session_test_fail() -> Result<Json<Option<User>>, errors::Error> {
    info!("No user in session!");
    Ok(Json(None))
}
