use diesel::{
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    errors,
    models::session::{NewSession, Session},
    SuccDb,
};

pub async fn create_session(db: &SuccDb, new_session: NewSession) -> Result<(), errors::Error> {
    use crate::schema::sessions::dsl::*;

    db.run(move |conn| {
        let _num_rows = diesel::insert_into(sessions)
            .values(&new_session)
            .execute(conn)
            .expect("Error creating new session");

        Ok(())
    })
    .await
}

pub async fn get_session_by_hash(
    db: &SuccDb,
    hash: String,
) -> Result<Option<Session>, errors::Error> {
    use crate::schema::sessions::dsl::*;

    db.run(|conn| {
        let rows = sessions
            .limit(1)
            .filter(jwt_hash.eq(hash))
            .select(Session::as_select())
            .load(conn)
            .expect("failed to get session by hash");

        if rows.is_empty() {
            return Ok(None);
        }

        let session = rows[0].clone();

        Ok(Some(session))
    })
    .await
}
