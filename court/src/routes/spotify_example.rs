use rocket::{serde::json::Json, State};

use super::auth0::SessionUser;
use crate::{errors, models::Popularity, services::spotify_example, SpotifyApi};

#[get("/artist-popularity/<id>")]
pub async fn get_artist_popularity(
    id: &str,
    spotify: &State<SpotifyApi>,
) -> Result<Json<Popularity>, errors::Error> {
    let res = spotify_example::get_artist_popularity(id, spotify).await?;
    Ok(Json(res))
}

#[get("/user-session-test")]
pub async fn user_session_test(user: SessionUser) -> Result<(), errors::Error> {
    info!(
        "If you see this, then the request guard worked! {}",
        user.user_sub
    );
    Ok(())
}

#[get("/user-session-test", rank = 2)]
pub async fn user_session_test_fail() -> Result<(), errors::Error> {
    info!("No user in session!");
    Ok(())
}
