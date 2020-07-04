use std::fs;
use serde::Deserialize;
use std::collections::BTreeMap;
use csv::StringRecord;

use crate::transformer::{Transformer, Transformation};
use linked_hash_map::LinkedHashMap;
use crate::printable_error::{PrintableError, ConfigParseError};
use crate::options::Variables;
use crate::worker::MaybeTransformationsChain;

type InputColumnIndexByName = BTreeMap<String, usize>;

type MaybeSomeTransformation = Result<Option<Transformation>, ConfigParseError>;


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Expression {
    Input { input: String },
    MultipleInput { input: Vec<String> },
    Trim { trim: usize },
    Replace { replace: LinkedHashMap<String, String> },
    Variable { var: String },
    Value { value: String },
    From { from: String },
    Date { date: String },
    Operation(String),
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Column {
    Input(String),
    Expressions(Vec<Expression>),
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


fn compile_multiple_input(
    input_column_names: &Vec<String>,
    input_column_index_by_name: &InputColumnIndexByName,
) -> MaybeSomeTransformation {
    let maybe_input_column_index: Option<&usize> = input_column_names.iter().map(
        |column_name| input_column_index_by_name.get(column_name),
    ).flatten().next();

    Ok(maybe_input_column_index.map(
        |index| Transformation::Input(index.clone()),
    ))
}


#[cfg(test)]
mod config_tests {
    use super::*;
    use serde_json::ser::State::Rest;

    fn test_compile_multiple_input_first() {
        let names = vec![
            "Date".to_string(),
            "date".to_string(),
        ];

        let mut indices = InputColumnIndexByName::new();

        indices.insert("date".to_string(), 2);
        indices.insert("Date".to_string(), 1);

        let transformation = compile_multiple_input(
            &names,
            &indices,
        ).unwrap().unwrap();

        assert_eq!(
            transformation,
            Transformation::Input(1),
        )
    }


    fn test_compile_multiple_input_second() {
        let names = vec![
            "Date".to_string(),
            "date".to_string(),
            "Transaction Date".to_string(),
        ];

        let mut indices = InputColumnIndexByName::new();

        indices.insert("Transaction Date".to_string(), 5);
        indices.insert("Event Date".to_string(), 1);

        let transformation = compile_multiple_input(
            &names,
            &indices,
        ).unwrap().unwrap();

        assert_eq!(
            transformation,
            Transformation::Input(5),
        )
    }

    fn test_compile_multiple_input_empty() {
        let names = vec![
            "Date".to_string(),
            "date".to_string(),
            "Transaction Date".to_string(),
        ];

        let mut indices = InputColumnIndexByName::new();

        indices.insert("Happening Date".to_string(), 5);
        indices.insert("Event Date".to_string(), 1);

        let maybe_some_transformation = compile_multiple_input(
            &names,
            &indices,
        );

        assert!(maybe_some_transformation.unwrap().is_none());
    }
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
    let value = variables.get(
        name,
    ).map(
        |value| value.clone(),
    ).unwrap_or(
        "".to_string(),
    );

    Ok(Some(Transformation::Value { value }))
}


fn from_transformation(name: &String) -> MaybeSomeTransformation {
    Ok(Some(Transformation::From { from: name.clone() }))
}

fn date_transformation(format: &String) -> MaybeSomeTransformation {
    Ok(Some(Transformation::Date { format: format.clone() } ))
}



fn expression_to_transformation(
    step: &Expression,
    input_column_index_by_name: &BTreeMap<String, usize>,
    variables: &Variables,
) -> MaybeSomeTransformation {
    match step {
        Expression::Input {input} => input_transformation(
            input,
            input_column_index_by_name,
        ),

        Expression::MultipleInput { input } => compile_multiple_input(
            input,
            input_column_index_by_name,
        ),

        Expression::Trim {trim} => Ok(Some(Transformation::Slice { start: 0, end: *trim })),

        Expression::Replace { replace } => Ok(Some(
            Transformation::Replace { replace: replace.clone() }
        )),

        Expression::Variable { var: variable } => variable_transformation(
            variable,
            variables,
        ),

        Expression::Value { value } => Ok(Some(
            Transformation::Value { value: value.clone() }
        )),

        Expression::From { from } => from_transformation(from),

        Expression::Date { date } => date_transformation(date),

        Expression::Operation(value) => transformation_without_parameters(
            value,
        )
    }
}


fn shorthand_input_to_transformations_chain(
    input_column_name: &String,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> MaybeTransformationsChain {
    let step = Expression::Input {
        input: input_column_name.clone(),
    };

    let maybe_some_transformation = expression_to_transformation(
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


fn expressions_to_transformations_chain(
    steps: &Vec<Expression>,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> MaybeTransformationsChain {
    let mapped_steps = steps.iter().map(
        |step| expression_to_transformation(
            step,
            &input_column_index_by_name,
            &variables,
        ),
    );

    let maybe_some_transformations: Result<Vec<Option<Transformation>>, ConfigParseError> = mapped_steps.collect();

    Ok(maybe_some_transformations?.into_iter().flatten().collect())
}


fn column_to_transformations_chain(
    column: &Column,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> MaybeTransformationsChain {
    match column {
        Column::Input(input_column_name) => shorthand_input_to_transformations_chain(
            input_column_name,
            input_column_index_by_name,
            variables,
        ),

        Column::Expressions(steps) => expressions_to_transformations_chain(
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
        |column| column_to_transformations_chain(
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
