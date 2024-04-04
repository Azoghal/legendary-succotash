use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub auth0subject: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, TS)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct User {
    pub id: i32,
    pub auth0subject: String,
    pub name: String,
}

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

// See ts-rs/example/src/lib.rs TODO:
//  1. Look at various serde-compatibility features in ts-rs
//  2. look at type renaming e.g. all lowercasing
//  3. consider: these will be generated any time we run test... which is a bit of a pain
#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct Popularity {
    pub popularity: u32,
}
