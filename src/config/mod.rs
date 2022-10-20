use toml;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub show_info: Show,
    pub branch_info: Branch,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Show {
    pub year: u32,
    pub show_type: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub judges: String,
}
#[derive(Deserialize, Debug)]
pub struct Branch {
    pub branch_name: String,
}

impl Config{
    pub fn _get_branch(&self) -> String{
        self.branch_info.branch_name.to_string()
    }
    pub fn _get_show_info(&self) -> Show {
        self.show_info.clone()
    }
}
pub fn load_config() -> Config{
    let raw_config = std::fs::read_to_string("show_info.toml").unwrap();
    let config: Config = toml::from_str(&raw_config).unwrap();
    config
}

mod tests {
    #[test]
    fn test_load() {
        let conf = super::load_config();
        assert!(conf._get_branch() == "Colorado Branch");
    }
    #[test]
    fn test_date() {
        let conf = super::load_config();
        assert!(conf.show_info.date == "Feb 2nd");
    }

}