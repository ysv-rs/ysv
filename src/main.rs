extern crate ysv;
use clap::clap_app;

use std::{env, error};
use ysv::{run, parse_config_from_file, LogFormat};


/// Parse command line arguments and start the application.
fn main() -> Result<(), String> {
    let matches = clap_app!(ysv =>
        (version: "0.1.6")
        (author: "Anatoly I. Scherbakov <altaisoft@gmail.com>")
        (about: "YAML-driven CSV formatter")
        (@arg LOG_FORMAT: -f --log-format +takes_value "Log format: 'plain' (default) or 'json'")
        (@arg CONFIG: +required "Sets the YAML configuration file to use")
        (@arg INPUT: "Sets the input CSV file(s) to read from")
    ).get_matches();

    let log_format = match matches.value_of("LOG_FORMAT").unwrap_or("plain") {
        "json" => LogFormat::JSON,
        "plain" => LogFormat::PLAIN,
        anything_else => panic!("Log format {} is not supported.", anything_else)
    };

    /// We can safely call .unwrap() here because CONFIG argument is required.
    /// If we had dependent types in Rust we could be able to express that.
    let config_file_path = matches.value_of("CONFIG").unwrap();

    /// If input file(s) are not provided we will read from stdin.
    let input_files = matches.value_of("INPUT").unwrap_or("");

    run(log_format, config_file_path, input_files)
}
