use csv::{StringRecord, ByteRecord};
use linked_hash_map::LinkedHashMap;


#[derive(Debug)]
pub enum Transformation {
    Input(usize),
    Slice { start: usize, end: usize },
    Replace { replace: LinkedHashMap<String, String> },
    Value { value: String },
    From { from: String },
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
    String::from_utf8(
        bytes.to_vec(),
    ).unwrap_or(
        "".to_string(),
    )
}


fn replace_with_mapping(value: String, mapping: &LinkedHashMap<String, String>) -> String {
    let mut result: String = value;

    for (from, to) in mapping.iter() {
        result = result.replace(from, to);
    }

    result
}


fn apply_input(row: &ByteRecord, index: &usize) -> Option<String> {
    row.get(*index).map(
        |bytes| safe_to_utf8(bytes),
    )
}


fn apply_line_number(line_number: usize) -> Option<String> {
    Some(line_number.to_string())
}


fn apply_from(column_name: String) -> Option<String> {
    Some(format!("{}? Ni!", column_name))
}


impl Transformation {
    pub fn apply(
        &self,
        value: Option<String>,
        row: &ByteRecord,
        line_number: usize,
    ) -> Option<String> {
        match self {
            Transformation::Input(index) => apply_input(row, index),

            // FIXME: this is a no-op still
            Transformation::Slice { start: _start, end: _end } => value,

            Transformation::Lowercase => value.map(
                |content| content.to_lowercase(),
            ),
            Transformation::Uppercase => value.map(
                |content| content.to_uppercase(),
            ),

            Transformation::Replace { replace } => value.map(
                |content| replace_with_mapping(
                    content,
                    replace,
                )
            ),

            Transformation::Value { value } => Some(value.clone()),

            Transformation::LineNumber => apply_line_number(line_number),

            Transformation::From { from } => apply_from(from.to_string()),
        }
    }
}
