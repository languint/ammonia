use logos::Logos;

#[derive(Debug, Logos, Clone, PartialEq)]
pub enum Token<'a> {
    Error,

    #[regex(r"[+-]?[0-9]+(\.[0-9]+)?")]
    Float(&'a str),

    #[regex(r"[ \t\f\n]+", logos::skip)]
    Whitespace,

    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
}
