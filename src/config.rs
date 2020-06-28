use std::fs;
use serde::Deserialize;
use std::collections::BTreeMap;
use csv::StringRecord;

use crate::transformer::{Transformer, Transformation};
use linked_hash_map::LinkedHashMap;
use crate::printable_error::{PrintableError, ConfigParseError};
use crate::options::Variables;

type InputColumnIndexByName = BTreeMap<String, usize>;

type MaybeSomeTransformation = Result<Option<Transformation>, ConfigParseError>;


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Step {
    Input { input: String },
    Trim { trim: usize },
    Replace { replace: LinkedHashMap<String, String> },
    Variable { var: String },
    Value { value: String },
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


pub fn parse_config_from_file(path: &str) -> Result<Config, PrintableError> {
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


fn input_transformation(
    input_column_name: &String,
    input_column_index_by_name: &InputColumnIndexByName,
) -> MaybeSomeTransformation {
    let input_column_index = input_column_index_by_name.get(
        input_column_name,
    );

    if input_column_index.is_none() {
        // FIXME this should not be here
        eprintln!("Warning: input column {} not found.", input_column_name);
    }

    Ok(input_column_index.map(
        |index| Transformation::Input(index.clone()),
    ))
}


fn transformation_without_parameters(
    transformation_name: &String,
) -> MaybeSomeTransformation {
    match transformation_name.as_str() {
        "uppercase" => Ok(Some(Transformation::Uppercase)),
        "lowercase" => Ok(Some(Transformation::Lowercase)),
        "line-number" => Ok(Some(Transformation::LineNumber)),
        _ => Err(ConfigParseError {
            column: None,
            transformation: Some(transformation_name.clone()),
            error_type: "unknown-transformation".to_string(),
            error_description: "This transformation is not supported. Please refer to documentation for the list of supported transformations.".to_string()
        })
    }
}


fn variable_transformation(
    name: &String,
    variables: &Variables,
) -> MaybeSomeTransformation {
    let value = match variables.get(name) {
        Some(value) => value.clone(),
        None => "".to_string(),
    };

    Ok(Some(Transformation::Value { value }))
}


fn step_to_transformation(
    step: &Step,
    input_column_index_by_name: &BTreeMap<String, usize>,
    variables: &Variables,
) -> Result<Option<Transformation>, ConfigParseError> {
    match step {
        Step::Input {input} => input_transformation(
            input,
            input_column_index_by_name,
        ),

        Step::Trim {trim} => Ok(Some(Transformation::Slice { start: 0, end: *trim })),

        Step::Replace { replace } => Ok(Some(
            Transformation::Replace { replace: replace.clone() }
        )),

        Step::Variable { var: variable } => variable_transformation(
            variable,
            variables,
        ),

        Step::Value { value } => Ok(Some(
            Transformation::Value { value: value.clone() }
        )),

        Step::Operation(value) => transformation_without_parameters(
            value,
        )
    }
}


fn shorthand_input_to_expressions(
    input_column_name: &String,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> Result<Vec<Transformation>, ConfigParseError> {
    let step = Step::Input {
        input: input_column_name.clone(),
    };

    let maybe_some_transformation = step_to_transformation(
        &step,
        input_column_index_by_name,
        variables,
    );

    maybe_some_transformation.map(
        |some_transformation| some_transformation.map_or(
            vec![],
            |transformation| vec![transformation],
        )
    )
}


fn steps_to_expressions(
    steps: &Vec<Step>,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> Result<Vec<Transformation>, ConfigParseError> {
    let mapped_steps = steps.iter().map(
        |step| step_to_transformation(
            step,
            &input_column_index_by_name,
            &variables,
        ),
    );

    let maybe_some_expressions: Result<Vec<Option<Transformation>>, ConfigParseError> = mapped_steps.collect();

    Ok(maybe_some_expressions?.into_iter().flatten().collect())
}


fn column_to_expressions(
    column: &Column,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> Result<Vec<Transformation>, ConfigParseError> {
    match column {
        Column::Input(input_column_name) => shorthand_input_to_expressions(
            input_column_name,
            input_column_index_by_name,
            variables,
        ),

        Column::Steps(steps) => steps_to_expressions(
            steps,
            input_column_index_by_name,
            variables,
        ),
    }
}


pub fn create_transformer(
    config: &Config,
    headers: &StringRecord,
    variables: &Variables,
) -> Result<Transformer, ConfigParseError> {
    let input_columns_index_by_name = get_input_columns_index_map(headers);

    let maybe_columns: Result<Vec<Vec<Transformation>>, ConfigParseError> = config.columns.values().map(
        |column| column_to_expressions(
            column,
            &input_columns_index_by_name,
            variables,
        ),
    ).collect();

    let headers = config.columns.keys().collect();

    Ok(Transformer {
        headers,
        columns: maybe_columns?,
    })
}
