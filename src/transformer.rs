use csv::{StringRecord, ByteRecord};

#[derive(Debug)]
pub enum Expression {
    Input(usize),
    Slice { start: usize, end: usize },
}


#[derive(Debug)]
pub struct Transformer {
    pub headers: StringRecord,
    pub columns: Vec<Vec<Expression>>,
}


impl Expression {
    pub fn apply(self, value: Option<String>, row: &ByteRecord) -> Option<String> {
        match self {
            Expression::Input(index) => Some(String::from(row[index])),
            Expression::Slice { start, end } => match value {
                Some(content) => Some(content),
                None => None
            }
        }
    }
}
