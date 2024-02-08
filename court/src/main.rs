use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate rocket;

mod routes;
mod services;

pub mod models;
pub mod schema;

#[cfg(test)]
mod tests;

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(&format!("{}/../lodge/dist", env!("CARGO_MANIFEST_DIR"))).join(file);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![files])
        .mount("/api/v1", routes![routes::succotash::get_recipes])
}
