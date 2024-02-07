use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

// use rocket::serde::{Deserialize, Serialize};
use super::establish_connection;
use crate::models::Recipe;

pub fn get_recipes() -> Vec<Recipe> {
    use crate::schema::recipes::dsl::*;

    // TODO lets bang the connection somewhere less repetitive?
    let connection = &mut establish_connection();
    recipes
        .limit(5)
        .select(Recipe::as_select())
        .load(connection)
        .expect("error loading recipes")
    //TODO handle error case
}
