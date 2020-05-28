use csv::{StringRecord, ByteRecord};
use linked_hash_map::LinkedHashMap;

#[derive(Debug)]
pub enum Expression {
    Input(usize),
    Slice { start: usize, end: usize },
    Replace { replace: LinkedHashMap<String, String> },
    Uppercase,
    Lowercase,
}


#[derive(Debug)]
pub struct Transformer {
    pub headers: StringRecord,
    pub columns: Vec<Vec<Expression>>,
}


fn safe_to_utf8(bytes: &[u8]) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value,
        Err(err) => String::new(),
    }
}


fn replace_with_mapping(value: String, mapping: &LinkedHashMap<String, String>) -> String {
    let mut result: String = value;

    for (from, to) in mapping.iter() {
        result = result.replace(from, to);
    }

    result
}


impl Expression {
    pub fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        match self {
            Expression::Input(index) => Some(safe_to_utf8(&row[*index])),
            Expression::Slice { start, end } => match value {
                Some(content) => Some(content),
                None => None,
            },

            Expression::Lowercase => match value {
                Some(content) => Some(content.to_lowercase()),
                None => None,
            },
            Expression::Uppercase => match value {
                Some(content) => Some(content.to_uppercase()),
                None => None,
            },

            Expression::Replace { replace } => match value {
                Some(content) => Some(replace_with_mapping(content, replace)),
                None => None,
            },
        }
    }
}
