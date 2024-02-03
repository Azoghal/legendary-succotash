use serde_derive::{Deserialize, Serialize};

pub fn get_recipes() -> Vec<Recipe> {
    let mut recipes = Vec::new();
    recipes.push(Recipe {
        recipe: "Step 1: Look up a recipe".to_string(),
    });
    recipes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub recipe: String,
}
