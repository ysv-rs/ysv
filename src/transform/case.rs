use crate::transform::{CellValue, ApplyResult};


/// Known string case transformations.
pub enum StringCase {
    Uppercase,
    Lowercase,
}


/// Change a string to the upper or lower case.
pub fn apply_change_case(value: CellValue, case: StringCase) -> ApplyResult {
    if let CellValue::String(Some(content)) = value {
        Ok(CellValue::String(Some(match case {
            StringCase::Lowercase => content.to_lowercase(),
            StringCase::Uppercase => content.to_uppercase(),
        })))

    } else {
        Err(format!(
            "Warning: cannot apply the case change transformation to a {} value '{:?}'.",
            &value.type_name(),
            &value,
        ))
    }
}
