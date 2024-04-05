use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

// TODO separate these out into separate files
#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct NewSession {
    pub user_id: i32,
    pub expires: i32,
    pub jwt_hash: String,
    pub jwt: String,
}

// TODO separate these out into separate files
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub expires: i32,
    pub jwt_hash: String,
    pub jwt: String,
}
