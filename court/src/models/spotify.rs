use rocket::serde::{Deserialize, Serialize};
use ts_rs::TS;

// See ts-rs/example/src/lib.rs TODO:
//  1. Look at various serde-compatibility features in ts-rs
//  2. look at type renaming e.g. all lowercasing
//  3. consider: these will be generated any time we run test... which is a bit of a pain
#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct Popularity {
    pub popularity: u32,
}
