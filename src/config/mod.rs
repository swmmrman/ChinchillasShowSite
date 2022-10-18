use toml;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct config {
    show_info: show,
    branch_info: branch,
}
#[derive(Deserialize, Debug)]
struct show {
    year: u32,
    show_type: String,
    date: String,
    start_time: String,
    end_time: String,
}
#[derive(Deserialize, Debug)]
struct branch {
    branch_name: String,
}

impl config{
    pub fn get_branch(&self) -> String{
        self.branch_info.branch_name.to_string()
    }
}
pub fn load_config() -> config{
    let raw_config = std::fs::read_to_string("show_info.toml").unwrap();
    let config: config = toml::from_str(&raw_config).unwrap();
    println!("{:?}", config);
    config
}