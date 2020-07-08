use crate::compile::models::{MaybeSomeTransformation, ReplaceRegex};
use crate::transform::Transformation;
use regex::Regex;
use crate::printable_error::ConfigParseError;


/// Accepts a number of mappings from regular expressions
pub fn compile_replace_regex(replace_regex: &ReplaceRegex) -> MaybeSomeTransformation {
    let pattern = Regex::new(replace_regex.pattern.as_str()).map_err(
        |err| ConfigParseError {
            column: None,
            transformation: Some("replace_regex".to_string()),
            error_type: "regex".to_string(),
            error_description: format!(
                "Cannot parse regular expression:\n\n  {}\n\nbecause: {}",
                replace_regex.pattern,
                err.to_string(),
            )
        }
    )?;

    let replace = replace_regex.replace.clone();

    Ok(Some(Transformation::ReplaceRegex {
        pattern,
        replace,
    }))
}
