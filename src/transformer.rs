#[derive(Debug)]
pub enum Expression {
    Input(usize),
    Slice { start: usize, end: usize },
}


#[derive(Debug)]
pub struct Transformer {
    pub columns: Vec<Vec<Expression>>,
}
