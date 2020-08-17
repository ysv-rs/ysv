use serde::Serialize;
use std::error::Error;
use std::fmt::Display;
use serde::export::Formatter;
use core::fmt;


#[derive(Debug, Serialize)]
pub struct PrintableError {
    pub error_type: String,
    pub error_description: String,
}


#[derive(Debug, Serialize)]
pub struct ConfigParseError {
    pub column: Option<String>,
    pub transformation: Option<String>,
    pub error_type: String,
    pub error_description: String,
}
