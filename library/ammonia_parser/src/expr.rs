#[derive(Debug, PartialEq)]
pub enum Expr {
    Error(String),
    Float(f64),
    Block(Vec<Self>),
}
