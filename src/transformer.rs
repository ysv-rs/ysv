use csv::{StringRecord, ByteRecord};
use linked_hash_map::LinkedHashMap;
use crate::options::Variables;

#[derive(Debug)]
pub enum Transformation {
    Input(usize),
    Slice { start: usize, end: usize },
    Replace { replace: LinkedHashMap<String, String> },
    Value { value: String },
    Uppercase,
    Lowercase,
    LineNumber,
}


#[derive(Debug)]
pub struct Transformer {
    pub headers: StringRecord,
    pub columns: Vec<Vec<Transformation>>,
}


fn safe_to_utf8(bytes: &[u8]) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value,
        Err(_err) => String::new(),
    }
}


fn replace_with_mapping(value: String, mapping: &LinkedHashMap<String, String>) -> String {
    let mut result: String = value;

    for (from, to) in mapping.iter() {
        result = result.replace(from, to);
    }

    result
}


fn input(row: &ByteRecord, index: &usize) -> Option<String> {
    match row.get(*index) {
        Some(bytes) => Some(safe_to_utf8(bytes)),
        None => None,
    }
}


fn apply_line_number(line_number: usize) -> Option<String> {
    Some(line_number.to_string())
}


impl Transformation {
    pub fn apply(
        &self,
        value: Option<String>,
        row: &ByteRecord,
        variables: &Variables,
        line_number: usize,
    ) -> Option<String> {
        match self {
            Transformation::Input(index) => input(row, index),
            Transformation::Slice { start: _start, end: _end } => match value {
                Some(content) => Some(content),
                None => None,
            },

            Transformation::Lowercase => match value {
                Some(content) => Some(content.to_lowercase()),
                None => None,
            },
            Transformation::Uppercase => match value {
                Some(content) => Some(content.to_uppercase()),
                None => None,
            },

            Transformation::Replace { replace } => match value {
                Some(content) => Some(replace_with_mapping(content, replace)),
                None => None,
            },

            Transformation::Value { value } => Some(value.clone()),

            Transformation::LineNumber => apply_line_number(line_number),
        }
    }
}
