use std::collections::BTreeMap;
use std::env;

use crate::compile::Config;

pub type Variables = BTreeMap<String, String>;


/// What is the format of the log messages we print out?
#[derive(Copy, Clone)]
pub enum LogFormat {
    /// For humans:
    PLAIN,

    /// For machines:
    JSON,
}


pub struct Options {
    pub(crate) config: Config,
    pub(crate) variables: Variables,
    pub(crate) input_files: Option<Vec<String>>,
}


// Fetch environment variables
pub fn determine_variables() -> Variables {
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
