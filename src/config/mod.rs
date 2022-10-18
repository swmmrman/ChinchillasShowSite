use toml;
use serde;

struct config {
    show_info: show,
    branch_info: branch,
}
struct show {
    year: u32,
    show_type: String,
    date: String,
    start_time: String,
    end_time: String,
}
struct branch {
    branch_name: String,
}

pub fn load_config() {
    let config = std::fs::read_to_string("show_info.toml").unwrap();
    println!("{:?}", config);
}