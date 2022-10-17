use std::path::Path;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::response::content;
use rocket::tokio::fs;


#[macro_use] extern crate rocket;

#[get("/css/<css_file>")]
async fn css(css_file: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/css/").join(css_file)).await.ok()
}

#[get("/")]
async fn index() -> content::RawHtml<String> {
    let index: String = fs::read_to_string(Path::new("template").join("index.html")).await.unwrap();
    let template = fs::read_to_string(Path::new("template").join("main.html")).await.unwrap();
    let output = template.replace("[content]", &index);
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


#[launch]
fn rocekt() -> _ {
    rocket::build()
        .mount("/", routes![index, css, js, def_route])
}