use serde::Deserialize;
use std::collections::BTreeMap;
use crate::transform::Transformation;
use crate::printable_error::ConfigParseError;
use linked_hash_map::LinkedHashMap;

pub type InputColumnIndexByName = BTreeMap<String, usize>;

pub type MaybeSomeTransformation = Result<Option<Transformation>, ConfigParseError>;


pub type ReplaceMapping = LinkedHashMap<String, String>;


#[derive(Debug, Deserialize)]
pub struct ReplaceRegex {
    pub pattern: String,
    pub replace: String,
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Expression {
    Input { input: String },
    MultipleInput { input: Vec<String> },

    Replace { replace: ReplaceMapping },
    ReplaceRegex { replace_regex: ReplaceRegex },

    Variable { var: String },
    Value { value: String },

    Date { date: String },
    MultipleDate { date: Vec<String> },

    Operation(String),

    // Not supported yet
    From { from: String },
    Trim { trim: usize },
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
