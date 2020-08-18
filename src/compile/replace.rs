use regex::Regex;

use crate::compile::models::{MaybeSomeTransformation, ReplaceRegex};
use crate::transform::Transformation;

/// Accepts a number of mappings from regular expressions
pub fn compile_replace_regex(replace_regex: &ReplaceRegex) -> MaybeSomeTransformation {
    let pattern = Regex::new(replace_regex.pattern.as_str()).map_err(
        |err| format!(
            "Cannot parse regular expression:\n\n  {}\n\nbecause: {}",
            replace_regex.pattern,
            err.to_string(),
        )
    )?;

    let replace = replace_regex.replace.clone();

    Ok(Some(Transformation::ReplaceRegex {
        pattern,
        replace,
    }))
}
