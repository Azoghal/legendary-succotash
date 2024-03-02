use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use super::establish_connection;
use crate::errors;
use crate::models::{NewUser, User};

pub fn create_user() -> Result<(), errors::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let new_user = NewUser { name: "Fred" };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    Ok(())
}
