use std::collections::BTreeMap;
use std::fs;

use csv::StringRecord;
use serde::Deserialize;

use crate::compile::input::{compile_multiple_input, compile_singular_input};
use crate::compile::models::{Column, Expression, InputColumnIndexByName, MaybeSomeTransformation};
pub use crate::compile::models::Config;
use crate::compile::replace::compile_replace_regex;
use crate::options::Variables;
use crate::printable_error::{ConfigParseError, PrintableError};
use crate::transform::{Transformation, Transformer};
use crate::worker::MaybeTransformationsChain;
use crate::compile::date::compile_date_with_multiple_formats;

mod input;
mod replace;
mod models;
mod date;


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

/// Create a specific date transformation based on the date format.
///
/// If format is "excel-ordinal", we will use particular algorithm for Excel ordinal dates;
/// otherwise, we will create a generic date transformation from normal formats. Thus, this will be
/// a compile time decision and we will not have to compare the format with a constant in runtime.
fn date_transformation(format: &String) -> MaybeSomeTransformation {
    Ok(Some(match format.as_str() {
        "excel-ordinal" => Transformation::ExcelOrdinalDate,
        _ => Transformation::Date { format: format.clone() }
    }))
}


fn compile_expression(
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

        Expression::ReplaceRegex { replace_regex } => compile_replace_regex(
            replace_regex,
        ),

        Expression::Variable { var: variable } => variable_transformation(
            variable,
            variables,
        ),

        Expression::Value { value } => Ok(Some(
            Transformation::Value { value: value.clone() }
        )),

        Expression::From { from } => from_transformation(from),

        Expression::Date { date } => date_transformation(date),
        Expression::MultipleDate { date } => compile_date_with_multiple_formats(date),

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

    let maybe_some_transformation = compile_expression(
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
        |step| compile_expression(
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
