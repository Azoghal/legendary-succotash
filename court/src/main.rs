use std::path::Path;

use rocket::fs::NamedFile;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod routes;
mod services;

#[cfg(test)]
mod tests;

// #[get("/")]
// fn index() -> Redirect {
//     Redirect::permanent("/index.html")
// }

#[get("/")]
async fn files() -> Option<NamedFile> {
    let path =
        Path::new(&format!("{}/../lodge/dist", env!("CARGO_MANIFEST_DIR"))).join("index.html");
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![files])
        .mount("api/v1", routes![routes::succotash::get_recipes])
}
