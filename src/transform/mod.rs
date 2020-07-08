use csv::{StringRecord, ByteRecord};
use linked_hash_map::LinkedHashMap;
use chrono::{NaiveDate, Duration};
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


fn apply_input(row: &ByteRecord, index: &usize) -> CellValue {
    CellValue::String(
        row.get(*index).map(
        |bytes| safe_to_utf8(bytes),
        )
    )
}


fn apply_line_number(line_number: usize) -> CellValue {
    CellValue::String(
        Some(line_number.to_string()),
    )
}


fn apply_from(column_name: String) -> CellValue {
    CellValue::String(
        Some(format!("{}? Ni!", column_name)),
    )
}


fn apply_lowercase(value: CellValue) -> CellValue {
    CellValue::String(
        match value {
            CellValue::String(maybe_string) => maybe_string.map(
                |content| content.to_lowercase(),
            ),

            _ => panic!("Runtime typing error: 'lowercase' transformation applied to {:?}.", value)
        }
    )
}


fn apply_uppercase(value: CellValue) -> CellValue {
    CellValue::String(
        match value {
            CellValue::String(maybe_string) => maybe_string.map(
                |content| content.to_uppercase(),
            ),

            _ => panic!("Runtime typing error: 'uppercase' transformation applied to {:?}.", value)
        }
    )
}


fn apply_replace(value: CellValue, mapping: &LinkedHashMap<String, String>) -> CellValue {
    CellValue::String(
        match value {
            CellValue::String(maybe_content) => maybe_content.map(
                |content| replace_with_mapping(
                    content,
                    mapping,
                )
            ),

            _ => panic!("Runtime typing error: 'replace' transformation applied to {:?}.", value)
        }
    )
}


/// Inspired by: https://stackoverflow.com/a/29387450/1245471
fn parse_excel_ordinal_date(value: String) -> Option<NaiveDate> {
    let maybe_ordinal: Option<i64> = value.parse().ok();

    if maybe_ordinal.is_none() {
        return None
    }

    let mut ordinal = maybe_ordinal.unwrap();
    let epoch = NaiveDate::from_ymd(1899, 12, 31);

    if ordinal >= 60 {
        ordinal = ordinal - 1;
    }

    Some(epoch + Duration::days(ordinal))
}


#[cfg(test)]
mod parse_excel_ordinal_date_tests {
    use super::*;

    fn test_38142() {
        let ordinal = 38142;
        let expected_date = NaiveDate::from_ymd(2004, 4, 6);
        let date = parse_excel_ordinal_date(ordinal.to_string()).unwrap();

        assert_eq!(date, expected_date);
    }
}


fn parse_date_with_format(value: String, format: &String) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(
        value.as_str(),
        format.as_str(),
    ).map_err(
        |err| eprintln!(
            "Cannot parse date {} with format {}.",
            value, format,
        )
    ).ok()
}


fn apply_parse_date(value: CellValue, format: &String) -> CellValue {
    CellValue::Date(
        match value {
            CellValue::String(maybe_content) => match maybe_content {
                Some(content) => {
                    if format == "excel-ordinal" {
                        parse_excel_ordinal_date(content)
                    } else {
                        parse_date_with_format(content, format)
                    }
                },

                // FIXME I do not understand how to do this without match right now
                None => None
            },

            _ => panic!("Runtime typing error: 'date' transformation applied to {:?}.", value),
        }
    )
}


fn apply_replace_regex(value: CellValue, regex: &Regex, replace: &String) -> CellValue {
    CellValue::String(
        match value {
            CellValue::String(maybe_content) => maybe_content.map(
                |content| regex.replace_all(
                        content.as_str(),
                        replace.as_str(),
                    ).to_string()
            ),

            _ => panic!(
                "Runtime typing error: 'replace_regex' transformation applied to {:?}.",
                value,
            ),
        }
    )
}


impl Transformation {
    pub fn apply(
        &self,
        value: CellValue,
        row: &ByteRecord,
        line_number: usize,
    ) -> CellValue {
        match self {
            Transformation::Input(index) => apply_input(row, index),

            // FIXME: this is a no-op still
            Transformation::Slice { start: _start, end: _end } => value,

            Transformation::Lowercase => apply_lowercase(value),
            Transformation::Uppercase => apply_uppercase(value),

            Transformation::Replace { replace } => apply_replace(
                value,
                replace,
            ),

            Transformation::ReplaceRegex {
                pattern, replace
            } => apply_replace_regex(
                value,
                pattern,
                replace,
            ),

            Transformation::Value { value } => CellValue::from_string(value.clone()),

            Transformation::LineNumber => apply_line_number(line_number),

            Transformation::From { from } => apply_from(from.to_string()),

            Transformation::Date { format } => apply_parse_date(value, format),
        }
    }
}
