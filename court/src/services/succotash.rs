use rocket::serde::{Deserialize, Serialize};

pub fn get_recipes() -> Vec<String> {
    vec!["Step 1: Look up a recipe".to_string()]
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub recipe: String,
}
