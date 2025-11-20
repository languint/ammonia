use crate::lexer::{error::LexerError, token::Token};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LexerResult {
    pub errors: Vec<LexerError>,
    pub tokens: Vec<Token>,
}

impl LexerResult {
    #[inline]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
