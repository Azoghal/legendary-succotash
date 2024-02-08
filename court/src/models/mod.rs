use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub instructions: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Recipes {
    pub recipes: Vec<Recipe>,
}
