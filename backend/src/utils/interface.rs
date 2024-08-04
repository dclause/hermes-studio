use std::path::Path;

use anyhow::Result;
use toml::Value as Toml;

pub struct Interface {}
impl Interface {
    pub fn get_config() -> Result<Toml> {
        let config_path = Path::new("./configs").join("interface.toml");
        let file_as_string =
            std::fs::read_to_string(config_path).unwrap_or_else(|_| String::default());
        let config: Toml = toml::from_str(&*file_as_string)?;
        Ok(config)
    }
    pub fn set_config(config: Toml) -> Result<Toml> {
        let config_path = Path::new("./configs").join("interface.toml");
        let config_as_string = toml::to_string(&config).unwrap();
        std::fs::write(config_path, config_as_string).unwrap();
        Ok(config)
    }
}
