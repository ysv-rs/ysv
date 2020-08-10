use crate::transform::CellValue;
use chrono::{NaiveDate, Duration};
use crate::printable_error::PrintableError;


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


pub fn apply_excel_ordinal_date(value: CellValue) -> CellValue {
    CellValue::Date(
        match value {
            CellValue::String(maybe_content) => maybe_content.map(
                |content| parse_excel_ordinal_date(content)
            ).unwrap_or(None),

            _ => panic!("Runtime typing error: 'excel_ordinal_date' transformation applied to {:?}.", value),
        }
    )
}


#[cfg(test)]
mod parse_excel_ordinal_date_tests {
    use super::*;

    #[cfg(test)]
    fn test_38142() {
        let ordinal = 38142;
        let expected_date = NaiveDate::from_ymd(2004, 4, 6);
        let date = apply_excel_ordinal_date(ordinal.to_string()).unwrap();

        assert_eq!(date, expected_date);
    }
}


fn parse_date_with_format(value: String, format: &String) -> Result<NaiveDate, PrintableError> {
    NaiveDate::parse_from_str(
        value.as_str(),
        format.as_str(),
    ).map_err(
        |_err| PrintableError {
            error_type: "date".to_string(),
            error_description: format!(
                "Cannot parse date {} with format {}.",
                value, format,
            ).to_string()
        }
    )
}


pub fn apply_parse_date(value: CellValue, format: &String) -> CellValue {
    CellValue::Date(
        match value {
            CellValue::String(maybe_content) => match maybe_content {
                Some(content) => {
                    parse_date_with_format(content, format).map_err(
                        |err| eprintln!("{}", err.error_description)
                    ).ok()
                },

                // FIXME I do not understand how to do this without match right now
                None => None
            },

            _ => panic!("Runtime typing error: 'date' transformation applied to {:?}.", value),
        }
    )
}


fn parse_date_with_formats(
    value: String,
    formats: &Vec<String>,
) -> Option<NaiveDate> {
    formats.iter().map(
        |format| parse_date_with_format(value.clone(), &format)
    ).flatten().next()
}


pub fn apply_date_multiple_formats(value: CellValue, formats: &Vec<String>) -> CellValue {
    CellValue::Date(
        match value {
            CellValue::String(maybe_content) => maybe_content.map(
                |content| parse_date_with_formats(content, formats)
            ).unwrap_or(None),

            _ => panic!("Runtime typing error: 'date' transformation applied to {:?}.", value),
        }
    )
}
