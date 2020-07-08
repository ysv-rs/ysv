use crate::transform::CellValue;


pub fn apply_lowercase(value: CellValue) -> CellValue {
    CellValue::String(
        match value {
            CellValue::String(maybe_string) => maybe_string.map(
                |content| content.to_lowercase(),
            ),

            _ => panic!("Runtime typing error: 'lowercase' transformation applied to {:?}.", value)
        }
    )
}


pub fn apply_uppercase(value: CellValue) -> CellValue {
    CellValue::String(
        match value {
            CellValue::String(maybe_string) => maybe_string.map(
                |content| content.to_uppercase(),
            ),

            _ => panic!("Runtime typing error: 'uppercase' transformation applied to {:?}.", value)
        }
    )
}
