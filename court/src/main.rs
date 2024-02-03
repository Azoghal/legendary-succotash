use std::path::{Path, PathBuf};

use rocket::{
    fs::{relative, FileServer, NamedFile},
    response::Redirect,
};

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

// #[get("/")]
// fn index() -> Redirect {
//     Redirect::permanent("/index.html")
// }

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(&format!("{}/../lodge", env!("CARGO_MANIFEST_DIR"))).join(file);
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![files])
}
