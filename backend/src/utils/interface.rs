use anyhow::Result;
use serde_json::Value;

use crate::utils::database::ArcDb;

pub struct Interface {}
impl Interface {
    pub fn get_config_from_db(database: ArcDb) -> Result<Value> {
        let config_as_string = database
            .read()
            .read_file("interface.json")
            .unwrap_or_else(|_| String::from("{}"));
        let config = serde_json::from_str(&*config_as_string)?;
        Ok(config)
    }
    pub fn set_config_to_db(database: ArcDb, config: Value) -> Result<Value> {
        let config_as_string = serde_json::to_string_pretty(&config).unwrap();
        database
            .write()
            .write_file("interface.json", config_as_string)
            .unwrap();
        Ok(config)
    }
}
