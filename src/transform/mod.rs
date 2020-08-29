mod models;
mod input;
mod replace;
mod case;
mod date;

use csv::ByteRecord;


pub use crate::transform::models::{
    Transformer,
    Transformation,
    CellValue,
    ApplyResult,
};
use crate::transform::input::apply_input;
use crate::transform::replace::{apply_replace, apply_replace_regex};
use crate::transform::case::{apply_uppercase, apply_lowercase};
use crate::transform::date::{apply_parse_date, apply_date_multiple_formats, apply_excel_ordinal_date};


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


impl Transformation {
    pub fn apply(
        &self,
        value: CellValue,
        row: &ByteRecord,
        line_number: usize,
    ) -> ApplyResult {
        match self {
            Transformation::Input(index) => Ok(apply_input(row, index)),

            // FIXME: this is a no-op still
            Transformation::Slice { start: _start, end: _end } => Ok(value),

            Transformation::Lowercase => Ok(apply_lowercase(value)),
            Transformation::Uppercase => Ok(apply_uppercase(value)),

            Transformation::Replace { replace } => Ok(apply_replace(
                value,
                replace,
            )),

            Transformation::ReplaceRegex {
                pattern, replace
            } => Ok(apply_replace_regex(
                value,
                pattern,
                replace,
            )),

            Transformation::Value { value } => Ok(CellValue::from_string(
                value.clone(),
            )),

            Transformation::LineNumber => Ok(apply_line_number(line_number)),

            Transformation::From { from } => Ok(apply_from(from.to_string())),

            Transformation::Date { format } => apply_parse_date(value, format),
            Transformation::DateMultiple { formats } => apply_date_multiple_formats(
                value, formats,
            ),
            Transformation::ExcelOrdinalDate => Ok(apply_excel_ordinal_date(value)),
        }
    }
}
