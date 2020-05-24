mod config;
mod worker;

use std::env;

use worker::process;
use config::parse_config_from_file;


fn main() {
    match env::args_os().nth(1) {
       None => eprintln!("Please specify configuration file."),
       Some(file_path) => match parse_config_from_file(file_path) {
           Ok(config) => process(config),
           Err(err) => eprintln!("{}", err)
       },
    }
}
