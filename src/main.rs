use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

#[get("/css/<css_file>")]
async fn css(css_file: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("css/").join(css_file)).await.ok()
}

#[get("/index.php")]
fn index() -> &'static str {
    "Hello"
}


#[launch]
fn rocekt() -> _ {
    rocket::build()
        .mount("/", routes![index, css])
}