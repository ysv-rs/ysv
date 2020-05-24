#[derive(Debug)]
pub enum Expression {
    Input(usize),
}


#[derive(Debug)]
pub struct Transformer {
    pub columns: Vec<Vec<Expression>>,
}
