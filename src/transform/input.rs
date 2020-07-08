use crate::transform::CellValue;
use csv::ByteRecord;


fn safe_to_utf8(bytes: &[u8]) -> String {
    String::from_utf8(
        bytes.to_vec(),
    ).unwrap_or(
        "".to_string(),
    )
}


pub fn apply_input(row: &ByteRecord, index: &usize) -> CellValue {
    CellValue::String(
        row.get(*index).map(
        |bytes| safe_to_utf8(bytes),
        )
    )
}
