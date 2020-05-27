use csv::{StringRecord, ByteRecord};

#[derive(Debug)]
pub enum Expression {
    Input(usize),
    Slice { start: usize, end: usize },
    Uppercase,
    Lowercase,
}


pub struct Input(usize);

pub struct Uppercase;
pub struct Lowercase;

pub struct Slice {
    start: usize,
    end: usize,
}

pub trait Expr {
    fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String>;
}


impl Expr for Input {
    fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        Some(safe_to_utf8(&row[self.0]))
    }
}


impl Expr for Lowercase {
    fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        match value {
            Some(string) => Some(string.to_lowercase()),
            None => None,
        }
    }
}


impl Expr for Uppercase {
    fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        match value {
            Some(string) => Some(string.to_uppercase()),
            None => None,
        }
    }
}


impl Expr for Slice {
    fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        match value {
            Some(content) => Some(content),
            None => None,
        }
    }
}


#[derive(Debug)]
pub struct Transformer {
    pub headers: StringRecord,
    pub columns: Vec<Vec<Expression>>,
}


fn safe_to_utf8(bytes: &[u8]) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value,
        Err(err) => String::new(),
    }
}


impl Expression {
    pub fn apply(&self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        match self {
            Expression::Input(index) => Some(safe_to_utf8(&row[*index])),
            Expression::Slice { start, end } => match value {
                Some(content) => Some(content),
                None => None,
            },

            Expression::Lowercase => match value {
                Some(content) => Some(content.to_lowercase()),
                None => None,
            },
            Expression::Uppercase => match value {
                Some(content) => Some(content.to_uppercase()),
                None => None,
            },
        }
    }
}
