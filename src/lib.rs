pub use crate::compile::parse_config_from_file;
use crate::options::{determine_variables, Options};
pub use crate::options::LogFormat;
use crate::worker::process;

mod worker;
mod options;
mod compile;
mod transform;
mod writer;

/// Configure the logger which will print log to stderr.
/// Well, it is currently no-op
fn configure_logging(_log_format: LogFormat) -> () {
    simple_logger::init().unwrap();
}


/// Run ysv from command line.
pub fn run(
    log_format: LogFormat,
    config_file_path: &str,
    input_files: Option<Vec<String>>,
) -> Result<(), String> {
    let config = parse_config_from_file(config_file_path)?;
    let variables = determine_variables();

    // A little dirty side-effect: set logging format
    configure_logging(log_format);

    process(Options { config, variables, input_files })
}
