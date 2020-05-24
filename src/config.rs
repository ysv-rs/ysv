use std::fs;
use std::ffi::OsString;
use serde::Deserialize;
use std::collections::BTreeMap;
use csv::StringRecord;


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Input { input: String },
    Trim { trim: u16 },
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Column {
    Input(String),
    Steps(Vec<Step>),
}


#[derive(Debug, Deserialize)]
pub struct Config {
    version: i8,
    columns: BTreeMap<String, Column>,
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


pub fn create_transformer(config: &Config, headers: &StringRecord) -> Transformer {
    let input_columns_index_by_name = get_input_columns_index_map(headers);
    eprintln!("Index by name: {:?}", input_columns_index_by_name);

    for (output_column_name, column) in config.columns.iter() {
        let column_transformations = match column {
            Column::Input(raw_column_name) => match input_columns_index_by_name.get(
                raw_column_name
            ) {
                Some(index) => Ok(vec![Expression::Input(index.clone())]),
                None => Err(format!("Column {} is not found in the input file.", raw_column_name))
            },
            Column::Steps(transformations) => Ok(vec![])
        };
    }

    Transformer {
        columns: vec![]
    }
}
