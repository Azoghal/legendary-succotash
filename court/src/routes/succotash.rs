use rocket::serde::json::{json, Value};

use crate::services::succotash;

#[get("/recipes")]
pub fn get_recipes() -> Value {
    json!({"recipes":succotash::get_recipes()})
}
