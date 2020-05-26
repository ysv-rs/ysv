mod config;
mod worker;
mod transformer;

use std::env;

use worker::process;
use config::parse_config_from_file;


fn main() -> Result<(), String> {
    let file_path = env::args_os().nth(1).expect(
        "Please specify configuration file path.",
    );
    let config = parse_config_from_file(file_path)?;

    Ok(process(config)?)
}
