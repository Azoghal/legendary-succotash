use diesel::{
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    errors,
    models::{NewSession, Session},
    services::establish_connection,
};

pub fn create_session(new_session: NewSession) -> Result<(), errors::Error> {
    use crate::schema::sessions::dsl::*;

    let connection = &mut establish_connection();

    let _num_rows = diesel::insert_into(sessions)
        .values(&new_session)
        .execute(connection)
        .expect("Error creating new session");

    Ok(())
}

pub fn get_session_by_hash(hash: String) -> Result<Option<Session>, errors::Error> {
    use crate::schema::sessions::dsl::*;

    let connection = &mut establish_connection();

    let rows = sessions
        .limit(1)
        .filter(jwt_hash.eq(hash))
        .select(Session::as_select())
        .load(connection)
        .expect("failed to get session by hash");

    if rows.is_empty() {
        return Ok(None);
    }

    let session = rows[0].clone();

    Ok(Some(session))
}
