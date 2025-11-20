use crate::lexer::defs::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Float(f64, Span),
    Int(i64, Span),
    LeftBrace(Span),
    RightBrace(Span),
}
