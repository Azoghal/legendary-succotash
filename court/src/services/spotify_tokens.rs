use diesel::{RunQueryDsl, SelectableHelper};

use super::establish_connection;
use crate::errors;
use crate::models::spotify::SpotifyToken;

pub fn create_spotify_token(
    user_id: i32,
    token_text: String,
) -> Result<SpotifyToken, errors::Error> {
    use crate::schema::spotify_tokens::dsl::spotify_tokens;

    let connection = &mut establish_connection();

    let new_token = SpotifyToken {
        user_id,
        token: token_text,
    };

    let res = diesel::insert_into(spotify_tokens)
        .values(new_token)
        .returning(SpotifyToken::as_returning())
        .get_result(connection)?;

    Ok(res)
}
