use diesel::RunQueryDsl;

use crate::{errors, models::Session, services::establish_connection};

pub fn create_session(new_session: Session) -> Result<(), errors::Error> {
    use crate::schema::sessions::dsl::*;

    let connection = &mut establish_connection();

    let _num_rows = diesel::insert_into(sessions)
        .values(&new_session)
        .execute(connection)
        .expect("Error creating new session");

    Ok(())
}
