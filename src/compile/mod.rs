mod input;
mod models;

use std::fs;
use serde::Deserialize;
use std::collections::BTreeMap;
use csv::StringRecord;

use crate::transformer::{Transformer, Transformation};
use linked_hash_map::LinkedHashMap;
use crate::printable_error::{PrintableError, ConfigParseError};
use crate::options::Variables;
use crate::worker::MaybeTransformationsChain;
use crate::compile::input::{compile_multiple_input, compile_singular_input};
use crate::compile::models::{InputColumnIndexByName, MaybeSomeTransformation, Expression, Column};

pub use crate::compile::models::Config;


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
        Expression::Input {input} => compile_singular_input(
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
    expressions: &Vec<Expression>,
    input_column_index_by_name: &InputColumnIndexByName,
    variables: &Variables,
) -> MaybeTransformationsChain {
    let mapped_steps = expressions.iter().map(
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
