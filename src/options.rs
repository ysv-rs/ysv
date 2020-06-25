use crate::config::Config;
use std::collections::BTreeMap;

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
