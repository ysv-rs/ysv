use std::fs;
use std::ffi::OsString;
use serde::Deserialize;
use std::collections::BTreeMap;
use csv::StringRecord;

use crate::transformer::{Transformer, Expression};


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Input { input: String },
    Trim { trim: usize },
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


fn get_input_columns_index_map(headers: &StringRecord) -> BTreeMap<String, usize> {
    // FIXME awful function, I do not know the proper method yet
    let mut mapping = BTreeMap::new();

    for (index, value) in headers.iter().enumerate() {
        mapping.insert(String::from(value), index);
    }

    mapping
}


fn step_to_expression(
    step: &Step,
    input_column_index_by_name: &BTreeMap<String, usize>,
) -> Result<Expression, String> {
    match step {
        Step::Input {input} => match input_column_index_by_name.get(input) {
            Some(index) => Ok(Expression::Input(index.clone())),
            None => Err(format!("Input column {} not found.", input))
        },

        Step::Trim {trim} => Ok(Expression::Slice { start: 0, end: *trim })
    }
}


fn column_to_expressions(
    column: &Column,
    input_column_index_by_name: &BTreeMap<String, usize>,
) -> Result<Vec<Expression>, String> {
    match column {
        Column::Input(input_column_name) => match step_to_expression(
            &Step::Input {
                input: input_column_name.clone(),
            },
            &input_column_index_by_name,
        ) {
            Ok(expression) => Ok(vec![expression]),
            Err(err) => Err(err),
        },
        Column::Steps(steps) => steps.iter().map(|step| step_to_expression(
            step,
            &input_column_index_by_name,
        )).collect()
    }
}


pub fn create_transformer(config: &Config, headers: &StringRecord) -> Result<Transformer, String> {
    let input_columns_index_by_name = get_input_columns_index_map(headers);
    eprintln!("Index by name: {:?}", input_columns_index_by_name);

    let maybe_columns: Result<Vec<Vec<Expression>>, String> = config.columns.values().map(
        |column| column_to_expressions(
            column,
            &input_columns_index_by_name,
        ),
    ).collect();

    match maybe_columns {
        Ok(columns) => Ok(Transformer { columns }),
        Err(err) => Err(err)
    }
}
