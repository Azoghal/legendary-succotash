use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use super::establish_connection;
use crate::errors;
use crate::models::users::{NewUser, User};
use crate::schema::users::star;

pub fn create_user(new_user: NewUser) -> Result<User, errors::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let res = diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error creating new user");

    Ok(res)
}

pub fn get_user(user_id: i32) -> Result<User, errors::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let result = users.find(user_id).first(connection)?;

    Ok(result)
}

pub fn get_user_by_auth0_subject(auth0_sub: &str) -> Result<Option<User>, errors::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let rows: Vec<User> = users
        .limit(1)
        .filter(auth0subject.eq(auth0_sub))
        .select(User::as_select())
        .load(connection)?;

    if rows.is_empty() {
        return Ok(None);
    }

    let user = rows[0].clone();

    Ok(Some(user))
}

pub fn get_or_create_user(new_user: NewUser) -> Result<User, errors::Error> {
    let user_option = get_user_by_auth0_subject(new_user.auth0subject)?;

    match user_option {
        Some(user) => {
            info!("existing user login");
            Ok(user)
        }
        None => {
            info!("new user added and logged in");
            create_user(new_user)
        }
    }
}

pub fn set_user_refresh_token(
    user_id: i32,
    new_refresh_token: Option<String>,
) -> Result<(), errors::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    diesel::update(users.filter(id.eq(user_id)))
        .set(spotify_refresh_token.eq(new_refresh_token))
        .execute(connection);

    Ok(())
}
