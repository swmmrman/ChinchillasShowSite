use toml;
use rocket::serde;

struct config {
    show_info: show,
    branch_info: branch,
}
struct show {
    
}
struct branch {

}

pub fn load_config() {
    let config = std::fs::read_to_string("show_info.toml").unwrap();
    println!("{:?}", config);
}