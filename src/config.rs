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
) -> Expression {
    Expression::Input(5)
}


fn column_to_expressions(
    column: &Column,
    input_column_index_by_name: &BTreeMap<String, usize>,
) -> Vec<Expression> {
    match column {
        Column::Input(input_column_name) => vec![step_to_expression(
            &Step::Input {
                input: input_column_name.clone(),
            },
            &input_column_index_by_name,
        )],
        Column::Steps(steps) => steps.iter().map(|step| step_to_expression(
            step,
            &input_column_index_by_name,
        )).collect()
    }
}


pub fn create_transformer(config: &Config, headers: &StringRecord) -> Result<Transformer, String> {
    let input_columns_index_by_name = get_input_columns_index_map(headers);
    eprintln!("Index by name: {:?}", input_columns_index_by_name);

    let expressions: Vec<Vec<Expression>> = config.columns.values().map(
        |column| column_to_expressions(
            column,
            &input_columns_index_by_name,
        ),
    ).collect();


    eprintln!("Expressions list: {:?}", expressions);

    // for (output_column_name, column) in config.columns.iter() {
    //     let column_transformations = match column {
    //         Column::Input(raw_column_name) => match input_columns_index_by_name.get(
    //             raw_column_name
    //         ) {
    //             Some(index) => Ok(vec![Expression::Input(index.clone())]),
    //             None => Err(format!("Column {} is not found in the input file.", raw_column_name))
    //         },
    //         Column::Steps(steps) => Ok(steps.iter().map(step_to_expression))
    //     };
    // }

    Ok(Transformer {
        columns: vec![]
    })
}
