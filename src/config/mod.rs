use toml;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    show_info: Show,
    branch_info: Branch,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Show {
    year: u32,
    show_type: String,
    date: String,
    start_time: String,
    end_time: String,
}
#[derive(Deserialize, Debug)]
struct Branch {
    branch_name: String,
}

impl Config{
    pub fn get_branch(&self) -> String{
        self.branch_info.branch_name.to_string()
    }
    pub fn get_show_info(&self) -> Show {
        self.show_info.clone()
    }
}
pub fn load_config() -> Config{
    let raw_config = std::fs::read_to_string("show_info.toml").unwrap();
    let config: Config = toml::from_str(&raw_config).unwrap();
    println!("{:?}", config);
    config
}