use ammonia_errors::{codes::ErrCode, error::AmmoniaErr, severity::ErrSeverity};

use ammonia_defs::source::Span;

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

impl<'a> From<LexerError> for AmmoniaErr {
    fn from(err: LexerError) -> Self {
        let code = err.to_code();

        let severity = match &err {
            LexerError::InvalidCharacter(_, _) => ErrSeverity::Error,
            LexerError::InvalidFloat(kind, _) => match kind {
                InvalidFloatError::Overflow => ErrSeverity::Error,
                InvalidFloatError::TrailingDecimal | InvalidFloatError::LeadingDecimal => {
                    ErrSeverity::Warning
                }
            },
            LexerError::InvalidInt(InvalidIntError::Overflow, _) => ErrSeverity::Error,
        };

        AmmoniaErr {
            code,
            severity,
            spans: err.get_span().map_or(vec![], |s| vec![s.clone()]),
        }
    }
}

impl LexerError {
    #[must_use]
    pub fn get_span(&self) -> Option<&Span> {
        match self {
            Self::InvalidCharacter(_, s) => Some(s),
            Self::InvalidFloat(_, s) => Some(s),
            Self::InvalidInt(_, s) => Some(s),
        }
    }

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
