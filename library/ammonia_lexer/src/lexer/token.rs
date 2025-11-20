use crate::lexer::defs::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Float(f64, Span),
    Int(i64, Span),

    Plus(Span),
    Minus(Span),
    Star(Span),
    Backslash(Span),
    Slash(Span),
    Equal(Span),

    EqualEqual(Span),
    BangEqual(Span),
    LessEqual(Span),
    GreaterEqual(Span),
    Arrow(Span),
    FatArrow(Span),

    LeftParen(Span),
    RightParen(Span),

    LeftBrace(Span),
    RightBrace(Span),
}
