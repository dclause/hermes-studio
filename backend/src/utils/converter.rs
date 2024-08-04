use serde_json::Value as Json;
use toml::{Table, Value as Toml};

pub fn toml_to_json(toml: Toml) -> Json {
    match toml {
        Toml::String(s) => Json::String(s),
        Toml::Integer(i) => Json::Number(i.into()),
        Toml::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => Json::Bool(b),
        Toml::Array(arr) => Json::Array(arr.into_iter().map(toml_to_json).collect()),
        Toml::Table(table) => Json::Object(
            table
                .into_iter()
                .map(|(k, v)| (k, toml_to_json(v)))
                .collect(),
        ),
        Toml::Datetime(dt) => Json::String(dt.to_string()),
    }
}

// Convert serde_json::Value to Toml
pub fn json_to_toml(json: Json) -> Result<Toml, String> {
    match json {
        Json::String(s) => Ok(Toml::String(s)),
        Json::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Toml::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Toml::Float(f))
            } else {
                Err("Number is neither integer nor float".to_string())
            }
        }
        Json::Bool(b) => Ok(Toml::Boolean(b)),
        Json::Array(arr) => {
            let result: Result<Vec<Toml>, _> = arr.into_iter().map(json_to_toml).collect();
            result.map(Toml::Array)
        }
        Json::Object(map) => {
            let mut table = Table::new();
            for (k, v) in map {
                table.insert(k, json_to_toml(v)?);
            }
            Ok(Toml::Table(table))
        }
        _ => Err("Unsupported JSON value".to_string()),
    }
}
