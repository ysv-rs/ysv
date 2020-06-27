use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct PrintableError {
    pub error_type: String,
    pub error_description: String,
}


#[derive(Debug, Serialize)]
pub struct ConfigParseError {
    pub column_name: String,
    pub error_type: String,
    pub error_description: String,
}
