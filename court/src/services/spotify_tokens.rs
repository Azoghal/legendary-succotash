use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::spotify::SpotifyToken;
use crate::{errors, SuccDb};

pub async fn create_spotify_token(
    db: &SuccDb,
    user_id: i32,
    token_text: String,
) -> Result<SpotifyToken, errors::Error> {
    use crate::schema::spotify_tokens::dsl::{
        spotify_tokens, token as table_token, user_id as u_id,
    };

    db.run(move |conn| {
        let new_token = SpotifyToken {
            user_id,
            token: token_text.clone(),
        };

        let res = diesel::insert_into(spotify_tokens)
            .values(new_token)
            .on_conflict(u_id)
            .do_update()
            .set(table_token.eq(token_text))
            .returning(SpotifyToken::as_returning())
            .get_result(conn)?;

        Ok(res)
    })
    .await
}

pub async fn get_user_token(db: &SuccDb, u_id: i32) -> Result<Option<SpotifyToken>, errors::Error> {
    use crate::schema::spotify_tokens::dsl::{spotify_tokens, user_id};

    db.run(move |conn| {
        let rows: Vec<SpotifyToken> = spotify_tokens
            .limit(1)
            .filter(user_id.eq(u_id))
            .select(SpotifyToken::as_select())
            .load(conn)?;

        if rows.is_empty() {
            return Ok(None);
        }

        let token = rows[0].clone();

        Ok(Some(token))
    })
    .await
}
