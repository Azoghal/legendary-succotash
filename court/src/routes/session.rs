use rocket::serde::json::Json;

use crate::{errors, models::User, services::users::get_user_by_auth0_subject};

use super::auth0::SessionUser;

// TODO move these to own set of routes
#[get("/session_user")]
pub async fn session_user(user: SessionUser) -> Result<Json<Option<User>>, errors::Error> {
    info!(
        "If you see this, then the request guard worked! {}",
        user.user_sub
    );
    let user = get_user_by_auth0_subject(&user.user_sub)?;
    Ok(Json(user))
}

#[get("/session_user", rank = 2)]
pub async fn session_user_fail() -> Result<Json<Option<User>>, errors::Error> {
    info!("No user in session!");
    Ok(Json(None))
}
