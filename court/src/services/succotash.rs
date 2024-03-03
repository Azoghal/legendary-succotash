use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use super::establish_connection;
use crate::errors;
use crate::models::{Recipe, Recipes};

pub fn get_recipes() -> Result<Recipes, errors::Error> {
    use crate::schema::recipes::dsl::*;

    // TODO lets bang the connection somewhere less repetitive? Like a context for the requests
    let connection = &mut establish_connection();
    let result = recipes
        .limit(5)
        .select(Recipe::as_select())
        .load(connection)?;

    Ok(Recipes { recipes: result })
}
