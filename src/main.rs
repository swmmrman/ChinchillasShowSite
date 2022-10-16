use std::path::Path;
use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

#[get("/css/<css_file>")]
async fn css(css_file: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/css/").join(css_file)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("template").join("main.html")).await.ok()
}

#[get("/js/<js_file>")]
async fn js(js_file: &str) -> Option<NamedFile> {
    NamedFile::open(Path::new("public_html/js").join(js_file)).await.ok()
}

#[get("/<req>")]
async fn def_route(req: &str) -> Option<NamedFile>{
    if req == "index.html" { 
        index().await
    }
    else {
        //Todo add a proper 404 return
        index().await
    }
}


#[launch]
fn rocekt() -> _ {
    rocket::build()
        .mount("/", routes![index, css, js, def_route])
}