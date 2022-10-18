//use toml;
//use serde;

pub fn load_config() {
    let config = std::fs::read_to_string("show_info.toml").unwrap();
    println!("{:?}", config);
}