use rocket::serde::json::Json;

use crate::errors::Error;
use crate::models::Recipes;
use crate::services::succotash;

#[get("/recipes")]
pub fn get_recipes() -> Result<Json<Recipes>, Error> {
    let res = succotash::get_recipes()?;
    Ok(Json(res))
}
