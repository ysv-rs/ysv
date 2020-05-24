use std::fs;
use std::ffi::OsString;
use serde::Deserialize;
use std::collections::BTreeMap;


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Transformation {
    Input { input: String },
    Trim { trim: u16 },
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Column {
    Input(String),
    Transformations(Vec<Transformation>),
}


#[derive(Debug, Deserialize)]
pub struct Config {
    version: i8,
    pub(crate) columns: BTreeMap<String, Column>,
}


pub fn parse_config_from_file(path: OsString) -> Result<Config, String> {
    match fs::read_to_string(path) {
        Ok(content) => match serde_yaml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(err) => Err(format!("Could not parse YAML: {}", err.to_string()))
        },
        Err(err) => Err(format!("Cannot open config: {}", err.to_string()))
    }
}
