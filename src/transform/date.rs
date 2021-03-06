use crate::transform::CellValue;
use chrono::{NaiveDate, Duration};
use crate::transform::models::ApplyResult;


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


fn parse_date_with_format(value: String, format: &String) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(
        value.as_str(),
        format.as_str(),
    ).map_err(
        |_err| format!(
            "Cannot parse date {} with format {}.",
            value, format,
        )
    )
}


pub fn apply_parse_date(value: CellValue, format: &String) -> ApplyResult {
    if let CellValue::String(Some(content)) = value {
        parse_date_with_format(content, format).map(
            |date| CellValue::Date(Some(date))
        )
    } else {
        Err(format!(
            "Warning: cannot apply 'date' transformation to a {} value '{:?}'.",
            &value.type_name(),
            &value,
        ))
    }
}


fn parse_date_with_formats(
    value: String,
    formats: &Vec<String>,
) -> Result<NaiveDate, String> {
    let maybe_date: Option<NaiveDate> = formats.iter().map(
        |format| parse_date_with_format(value.clone(), &format)
    ).flatten().next();

    maybe_date.ok_or(format!(
        "Value '{value}' could not be recognized as date in any of formats: {formats_list}",
        value=value,
        formats_list=formats.join(", "),
    ))
}


pub fn apply_date_multiple_formats(value: CellValue, formats: &Vec<String>) -> ApplyResult {
    if let CellValue::String(Some(content)) = value {
        parse_date_with_formats(content, formats).map(
            |date| CellValue::Date(Some(date))
        )
    } else {
        Err(format!(
            "Warning: cannot apply 'date' transformation to a {} value '{:?}'.",
            &value.type_name(),
            &value,
        ))
    }
}
