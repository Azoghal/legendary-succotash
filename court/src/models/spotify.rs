use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use rspotify::Token;
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

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct AuthUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(crate = "rocket::serde")]
#[ts(export)]
pub struct CurrentPlaying {
    pub title: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::spotify_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct SpotifyToken {
    pub token: String,
    pub user_id: i32,
}
