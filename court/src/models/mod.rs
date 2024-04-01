use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, TS)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub instructions: String,
}

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

// TODO Think a good approach is to determine some sort of naming convention
// or *namespacing* for structs that will be returned to the routes

// This all sort of falls into the conversation of if we should have another intermediary

// Routes -> just calls a single procedure
// those procedures, potentially call many db queries, but return a valid json serializable struct and can be typescriptified
// the db query functions - return basically exactly what the query returns, potentially a little bit of modification

// Then only the route side structs need to be serialisable and typescriptable.
// and only the DB side structs need to derive the DB traits.

// But this means that we'd need some basically identity functions that take from DB structs to identical but serialisable frontend structs
// HM.

// See ts-rs/example/src/lib.rs
// TODO Look at various serde-compatibility features in ts-rs
// TODO look at type renaming e.g. all lowercasing
// TODO these will be generated any time we run test... which is a bit of a pain
#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct Recipes {
    pub recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct Popularity {
    pub popularity: u32,
}
