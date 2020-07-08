use chrono::NaiveDate;
use csv::StringRecord;
use linked_hash_map::LinkedHashMap;
use regex::Regex;


#[derive(Debug)]
pub enum Transformation {
    Input(usize),
    Slice { start: usize, end: usize },
    Replace { replace: LinkedHashMap<String, String> },
    ReplaceRegex { pattern: Regex, replace: String },
    Value { value: String },
    From { from: String },
    Date { format: String },
    Uppercase,
    Lowercase,
    LineNumber,
}


#[derive(Debug)]
pub enum CellValue {
    String(Option<String>),
    Date(Option<NaiveDate>),
}


impl CellValue {
    pub fn empty_string() -> CellValue {
        CellValue::String(Some("".to_string()))
    }

    pub fn from_string(value: String) -> CellValue {
        CellValue::String(Some(value))
    }

    pub fn to_string(&self) -> String {
        let empty_string = "".to_string();

        match self {
            CellValue::String(maybe_value) => maybe_value.as_ref().unwrap_or(
                &empty_string,
            ).clone(),

            CellValue::Date(maybe_value) => maybe_value.map(
                |naive_date| naive_date.to_string(),
            ).unwrap_or(
                empty_string,
            ),
        }
    }
}


#[derive(Debug)]
pub struct Transformer {
    pub headers: StringRecord,
    pub columns: Vec<Vec<Transformation>>,
}
