use std::fs;
use std::ffi::OsString;
use serde::Deserialize;
use std::collections::BTreeMap;
use csv::StringRecord;

use crate::transformer::{Transformer, Expression};
use linked_hash_map::LinkedHashMap;


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Input { input: String },
    Trim { trim: usize },
    Operation(String),
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
    pub(crate) columns: LinkedHashMap<String, Column>,
}


pub fn parse_config_from_file(path: OsString) -> Result<Config, String> {
    let content = fs::read_to_string(&path).expect(
        "Cannot open configuration file."
    );

    Ok(serde_yaml::from_str(&content).expect(
        "YAML config could not be parsed."
    ))
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
) -> Result<Option<Expression>, String> {
    match step {
        Step::Input {input} => match input_column_index_by_name.get(input) {
            Some(index) => Ok(Some(Expression::Input(index.clone()))),
            None => {
                eprintln!("Input column {} not found.", input);
                Ok(None)
            }
        },

        Step::Trim {trim} => Ok(Some(Expression::Slice { start: 0, end: *trim })),
        Step::Operation(value) => match value.as_str() {
            "uppercase" => Ok(Some(Expression::Uppercase)),
            "lowercase" => Ok(Some(Expression::Lowercase)),
            _ => Err(format!("Unknown operation: '{}'", value))
        },
    }
}


fn column_to_expressions(
    column: &Column,
    input_column_index_by_name: &BTreeMap<String, usize>,
) -> Result<Vec<Expression>, String> {
    match column {
        Column::Input(input_column_name) => {
            let maybe_some_expression = step_to_expression(
                &Step::Input {
                    input: input_column_name.clone(),
                },
                &input_column_index_by_name,
            );

            match maybe_some_expression? {
                Some(expression) => Ok(vec![expression]),
                None => Ok(vec![])
            }
        },

        Column::Steps(steps) => {
            let maybe_some_expressions: Result<Vec<Option<Expression>>, String> = steps.iter().map(
                |step| step_to_expression(
                    step,
                    &input_column_index_by_name,
                ),
            ).collect();
            eprintln!("Maybe some expressions: {:?}", maybe_some_expressions);

            Ok(maybe_some_expressions?.into_iter().flatten().collect())
        }
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

    let headers = config.columns.keys().collect();

    Ok(Transformer {
        headers,
        columns: maybe_columns?,
    })
}
