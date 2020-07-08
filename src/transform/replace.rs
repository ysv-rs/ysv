use crate::transform::CellValue;
use linked_hash_map::LinkedHashMap;
use regex::Regex;


fn replace_with_mapping(value: String, mapping: &LinkedHashMap<String, String>) -> String {
    let mut result: String = value;

    for (from, to) in mapping.iter() {
        result = result.replace(from, to);
    }

    result
}


pub fn apply_replace(
    value: CellValue,
    mapping: &LinkedHashMap<String, String>,
) -> CellValue {
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


pub fn apply_replace_regex(
    value: CellValue,
    regex: &Regex,
    replace: &String,
) -> CellValue {
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
