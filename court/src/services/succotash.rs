use rocket::serde::{Deserialize, Serialize};

pub fn get_recipes() -> Recipes {
    Recipes {
        recipes: vec![
            "Step 1: Look up a recipe".to_string(),
            "Clean Succotash:\nStep 1: Buy an entire country worth of sweetcorn".to_string(),
        ],
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Recipes {
    pub recipes: Vec<String>,
}
