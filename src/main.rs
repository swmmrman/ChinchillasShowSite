use std::path::Path;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::response::content;
use rocket::State;
use rocket::tokio::fs;
use chrono::{self, Datelike};
mod config;

#[macro_use] extern crate rocket;

#[get("/css/<css_file>")]
async fn css(css_file: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/css/").join(css_file)).await.ok()
}

#[get("/")]
async fn index(show: &State<config::Config>) -> content::RawHtml<String> {
    let index: String = fs::read_to_string(Path::new("template").join("index.html")).await.unwrap();
    let template = fs::read_to_string(Path::new("template").join("main.html")).await.unwrap();
    let mut output = template.replace("[content]", &index);
    let year = chrono::Utc::now().date().year().to_string();
    output = output.replace("[year]", &year);
    output = output.replace("[show info]", "info");
    content::RawHtml(output)
}

#[get("/js/<js_file>")]
async fn js(js_file: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/js").join(js_file)).await.ok()
}

#[get("/<req>")]
async fn def_route(req: &str) -> Option<Redirect>{
    let indexs = ["index.html", "index.php"];
    if indexs.contains(&req)  { 
        Some(Redirect::to(uri!(index)))
    }
    else {
        None
    }
}

#[get("/images/<img>")]
async fn images(img: &str) -> Option<NamedFile> {
    if img == "logo.png" {
        if std::fs::metadata(Path::new("logo.png")).is_ok() {
            NamedFile::open(Path::new("logo.png")).await.ok()
        }
        else {
            NamedFile::open(Path::new("public_html/images/logo.png")).await.ok()
        }
    }
    else {
        NamedFile::open(Path::new("public_html/images/").join(img)).await.ok()
    }
}


// struct ShowData {
//     year: String,
//     show_type: String,
//     date: String,
//     start_time: String,
//     end_time: String,
// }

#[launch]
fn rocket() -> _ {
    let conf = config::load_config();
    rocket::build()
        .mount("/", routes![index, css, js, def_route, images])
        .manage(conf)
}