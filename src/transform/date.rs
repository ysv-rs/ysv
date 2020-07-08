use crate::transform::CellValue;
use chrono::{NaiveDate, Duration};


/// Inspired by: https://stackoverflow.com/a/29387450/1245471
fn parse_excel_ordinal_date(value: String) -> Option<NaiveDate> {
    // FIXME: this function is being chosen to execute in runtime, but in fact
    //   the question whether to call it or not depends on a special format value
    //   in config file. Thus, it should be a compile time decision.
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

    #[cfg(test)]
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
        |_err| eprintln!(
            "Cannot parse date {} with format {}.",
            value, format,
        )
    ).ok()
}


pub fn apply_parse_date(value: CellValue, format: &String) -> CellValue {
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
            CellValue::String(maybe_content) => match maybe_content {
                Some(content) => parse_date_with_formats(
                    content, formats,
                ),
                None => None,
            }

            _ => panic!("Runtime typing error: 'date' transformation applied to {:?}.", value),
        }
    )
}
