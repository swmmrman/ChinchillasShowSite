use toml;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    show_info: Show,
    branch_info: Branch,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Show {
    pub year: u32,
    pub show_type: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
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

mod tests {
    #[test]
    fn test_load() {
        let conf = super::load_config();
        assert!(conf.get_branch() == "Colorado Branch");
    }
    #[test]
    fn test_date() {
        let conf = super::load_config();
        assert!(conf.show_info.date == "Feb 2nd");
    }

}