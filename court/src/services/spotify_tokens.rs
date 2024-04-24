use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

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

pub fn get_user_token(u_id: i32) -> Result<Option<SpotifyToken>, errors::Error> {
    use crate::schema::spotify_tokens::dsl::{spotify_tokens, user_id};

    let connection = &mut establish_connection();

    let rows: Vec<SpotifyToken> = spotify_tokens
        .limit(1)
        .filter(user_id.eq(u_id))
        .select(SpotifyToken::as_select())
        .load(connection)?;

    if rows.is_empty() {
        return Ok(None);
    }

    let token = rows[0].clone();

    Ok(Some(token))
}
