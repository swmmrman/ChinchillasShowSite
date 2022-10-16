#[macro_use] extern crate rocket;

#[get("/index.php")]
fn index() -> &'static str {
    "Hello"
}


#[launch]
fn rocekt() -> _ {
    rocket::build()
        .mount("/", routes![index])
}