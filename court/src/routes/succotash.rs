use rocket::serde::json::{json, Value};

use crate::services::succotash;

#[get("/recipes")]
pub fn get_recipes() -> Value {
    json!(succotash::get_recipes())
}
