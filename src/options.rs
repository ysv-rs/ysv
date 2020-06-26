use std::env;
use std::collections::BTreeMap;
use serde_json::json;

use crate::config::{Config, parse_config_from_file};
use crate::printable_error::PrintableError;

pub type Variables = BTreeMap<String, String>;


pub enum ErrorFormat {
    // In what format to print the error log?
    HumanReadable,
    JSON,
}


pub struct Options {
    pub(crate) error_format: ErrorFormat,
    pub(crate) config: Config,
    pub(crate) variables: Variables,
}


fn format_error(error: &PrintableError, format: &ErrorFormat) -> String {
    match format {
        ErrorFormat::HumanReadable => format!(
            "ysv : Error {} | {}",
            error.error_type,
            error.error_description,
        ),
        ErrorFormat::JSON => json!(error).to_string()
    }
}


// Fetch environment variables
fn determine_variables() -> Variables {
    let prefix = "YSV_VAR_";

    env::vars().filter(
        |(variable, _)| variable.starts_with(prefix)
    ).map(
        |(variable, value)| (
            variable.replace(prefix, ""),
            value,
        )
    ).collect()
}


pub fn get_options(args: Vec<String>) -> Result<Options, String> {
    let error_format = ErrorFormat::JSON;
    let first_argument = args.get(1);

    if let None = first_argument {
        return Err(String::from("Please provide configuration file path."))
    }

    let variables = determine_variables();

    let config_result = parse_config_from_file(
        first_argument.unwrap(),
    );

    match config_result {
        Ok(config) => Ok(Options { error_format, config, variables }),
        Err(err) => Err(format_error(&err, &error_format))
    }
}
