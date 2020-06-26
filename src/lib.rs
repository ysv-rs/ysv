mod worker;
mod options;
mod config;
mod transformer;
mod printable_error;

use serde_json::json;
use std::env;

// use config::parse_config_from_file;

// use worker::process;
// use super::worker::process;
use crate::worker::process;

// use crate::worker::process;
use crate::printable_error::PrintableError;
use crate::config::{Config, parse_config_from_file};
use crate::options::{Options, ErrorFormat, Variables};


const HELP: &str = r#"
γsv: standardize and process CSV files

Syntax:

  cat original.csv | ysv configuration.yaml > result.csv

Error messages are printed to stderr.

To get errors in JSON format for integration with other tools:

  cat original.csv | ysv configuration.yaml > result.csv
"#;


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


fn get_options(args: Vec<String>) -> Result<Options, String> {
    let first_argument = args.get(1);

    if let None = first_argument {
        return Err(String::from("Please provide configuration file path."))
    }

    let variables = determine_variables();
    let config = parse_config_from_file(first_argument.unwrap())?;

    Ok(Options {
        error_format: ErrorFormat::JSON,
        config,
        variables,
    })
}


// Did the user request the built-in help message?
fn is_help(args: &Vec<String>) -> bool {
    match args.get(1) {
        Some(argument_value) => (
            argument_value.as_str() == "--help"
        ),
        _ => false,
    }
}


pub fn run(args: Vec<String>) -> Result<(), String> {
    if is_help(&args) {
        eprintln!("{}", HELP);
    } else {
        let options = get_options(args)?;
        process(options)
    }

    Ok(())
}
