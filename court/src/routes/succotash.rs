use rocket_contrib::json::{Json, JsonValue};

use crate::services::{self, succotash::Recipe};

#[get("/recipes")]
pub fn get_recipes() -> JsonValue {
    json!(services::succotash::get_recipes())
}
