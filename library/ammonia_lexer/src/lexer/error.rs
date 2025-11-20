use ammonia_errors::codes::ErrCode;

use crate::lexer::defs::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidFloatError {
    TrailingDecimal,
    LeadingDecimal,
    Overflow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidIntError {
    Overflow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char, Span),
    InvalidFloat(InvalidFloatError, Span),
    InvalidInt(InvalidIntError, Span),
}

impl LexerError {
    #[must_use] 
    pub fn to_code(&self) -> ErrCode {
        match self {
            Self::InvalidCharacter(_, _) => ErrCode::E0001,
            Self::InvalidFloat(err_type, _) => match err_type {
                InvalidFloatError::TrailingDecimal => ErrCode::E0010,
                InvalidFloatError::LeadingDecimal => ErrCode::E0011,
                InvalidFloatError::Overflow => ErrCode::E0012,
            },
            Self::InvalidInt(err_type, _) => match err_type {
                InvalidIntError::Overflow => ErrCode::E0020,
            },
        }
    }
}
