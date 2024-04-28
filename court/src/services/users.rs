use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::users::{NewUser, User};
use crate::{errors, SuccDb};

pub async fn create_user(db: &SuccDb, new_user: NewUser) -> Result<User, errors::Error> {
    use crate::schema::users::dsl::*;

    db.run(move |conn| {
        let res = diesel::insert_into(users)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)?;
        Ok(res)
    })
    .await
}

pub async fn get_user(db: &SuccDb, user_id: i32) -> Result<User, errors::Error> {
    use crate::schema::users::dsl::*;

    db.run(move |conn| {
        let result = users.find(user_id).first(conn)?;
        Ok(result)
    })
    .await
}

pub async fn get_user_by_auth0_subject(
    db: &SuccDb,
    auth0_sub: String,
) -> Result<Option<User>, errors::Error> {
    use crate::schema::users::dsl::*;

    db.run(move |conn| {
        let rows: Vec<User> = users
            .limit(1)
            .filter(auth0subject.eq(auth0_sub))
            .select(User::as_select())
            .load(conn)?;

        if rows.is_empty() {
            return Ok(None);
        }

        let user = rows[0].clone();

        Ok(Some(user))
    })
    .await
}

pub async fn get_or_create_user(db: &SuccDb, new_user: NewUser) -> Result<User, errors::Error> {
    let user_option = get_user_by_auth0_subject(db, new_user.auth0subject.clone()).await?;

    match user_option {
        Some(user) => {
            info!("existing user login");
            Ok(user)
        }
        None => {
            info!("new user added and logged in");
            create_user(db, new_user).await
        }
    }
}
