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
