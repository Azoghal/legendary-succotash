use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

// use rocket::serde::{Deserialize, Serialize};
use super::establish_connection;
use crate::models::{Recipe, Recipes};

pub fn get_recipes() -> Recipes {
    use crate::schema::recipes::dsl::*;

    // TODO lets bang the connection somewhere less repetitive? Like a context for the requests
    let connection = &mut establish_connection();
    let result = recipes
        .limit(5)
        .select(Recipe::as_select())
        .load(connection)
        .expect("error loading recipes");
    //TODO handle error case

    Recipes { recipes: result }
}
