mod config;
mod worker;
mod transformer;
mod printable_error;
use serde_json::json;

mod lib;

use std::env;

use worker::process;
use config::parse_config_from_file;
use crate::printable_error::PrintableError;
use std::collections::HashMap;


const HELP: &str = r#"
γsv: standardize and process CSV files

Syntax:

  cat original.csv | ysv configuration.yaml > result.csv

Error messages are printed to stderr.

To get errors in JSON format for integration with other tools:

  cat original.csv | ysv --json-errors configuration.yaml > result.csv
"#;


enum ErrorFormat {
    HumanReadable,
    JSON,
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
fn determine_variables() -> HashMap<String, String> {
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


fn run(file_path: &str) -> Result<(), PrintableError> {
    let variables = determine_variables();
    let config = parse_config_from_file(file_path)?;

    match process(config, variables) {
        Ok(()) => Ok(()),
        Err(err) => Err(PrintableError {
            error_type: String::from("unknown"),
            error_description: err,
        })
    }

}


// TODO: rename this function
fn ysv(args: Vec<String>) -> Result<(), String> {
    let file_path_error = PrintableError {
        error_type: String::from("argument"),
        error_description: String::from("Input file name not specified."),
    };

    let first_argument = args.get(1).expect(format_error(
        &file_path_error,
        &ErrorFormat::HumanReadable,
    ).as_str());

    if first_argument == "--help" {
        eprintln!("{}", HELP);
        return Ok(())
    }

    let error_format = match first_argument.as_str() {
        "--errors-in-json" => ErrorFormat::JSON,
        _ => ErrorFormat::HumanReadable,
    };

    let file_path = match error_format {
        ErrorFormat::JSON => args.get(2).expect(format_error(
            &file_path_error,
            &error_format,
        ).as_str()),
        _ => first_argument,
    };

    match run(file_path) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("{}", format_error(&err, &error_format));
            Ok(())
        }
    }
}


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    Ok(ysv(args)?)
}
