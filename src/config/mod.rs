use toml;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct config {
    show_info: show,
    branch_info: branch,
}
#[derive(Deserialize)]
struct show {
    year: u32,
    show_type: String,
    date: String,
    start_time: String,
    end_time: String,
}
#[derive(Deserialize)]
struct branch {
    branch_name: String,
}

pub fn load_config() {
    let raw_config = std::fs::read_to_string("show_info.toml").unwrap();
    let config = toml::from_str(raw_config);
    println!("{:?}", config);
}