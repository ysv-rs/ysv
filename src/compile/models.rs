use serde::Deserialize;
use std::collections::BTreeMap;
use crate::transformer::Transformation;
use crate::printable_error::ConfigParseError;
use linked_hash_map::LinkedHashMap;

pub type InputColumnIndexByName = BTreeMap<String, usize>;

pub type MaybeSomeTransformation = Result<Option<Transformation>, ConfigParseError>;


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
