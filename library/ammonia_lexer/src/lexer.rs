pub mod defs;
pub mod error;
pub mod token;

use token::Token;

use crate::lexer::{
    defs::Span,
    error::{InvalidFloatError, InvalidIntError, LexerError},
};

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

pub struct AmmoniaLexer {
    pub src: String,
}
impl AmmoniaLexer {
    pub fn from_src(src: String) -> Self {
        Self { src }
    }
}

impl AmmoniaLexer {
    pub fn parse(&self) -> LexerResult {
        let mut result = LexerResult::default();
        let mut chars = self.src.char_indices().peekable();

        while let Some((start_idx, c)) = chars.next() {
            match c {
                '{' => {
                    let span = Span {
                        range: start_idx..start_idx + 1,
                        slice: "{".to_string(),
                    };
                    result.tokens.push(Token::LeftBrace(span));
                }
                '}' => {
                    let span = Span {
                        range: start_idx..start_idx + 1,
                        slice: "}".to_string(),
                    };
                    result.tokens.push(Token::RightBrace(span));
                }
                '-' => {
                    if let Some(&(_, next_c)) = chars.peek() {
                        if next_c.is_digit(10) || next_c == '.' {
                            let mut number = "-".to_string();
                            let mut end_idx = start_idx + 1;
                            while let Some(&(idx, next_c)) = chars.peek() {
                                if next_c.is_digit(10) || next_c == '.' {
                                    number.push(next_c);
                                    end_idx = idx + 1;
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            AmmoniaLexer::lex_number(
                                &self.src,
                                start_idx,
                                number,
                                end_idx,
                                &mut result,
                            );
                            continue;
                        }
                    }
                    let span = Span {
                        range: start_idx..start_idx + 1,
                        slice: "-".to_string(),
                    };
                    result.errors.push(LexerError::InvalidCharacter('-', span));
                }
                '0'..='9' | '.' => {
                    let mut number = c.to_string();
                    let mut end_idx = start_idx + 1;
                    while let Some(&(idx, next_c)) = chars.peek() {
                        if next_c.is_digit(10) || next_c == '.' {
                            number.push(next_c);
                            end_idx = idx + 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    AmmoniaLexer::lex_number(&self.src, start_idx, number, end_idx, &mut result);
                }
                c if c.is_whitespace() => {}
                _ => {
                    let span = Span {
                        range: start_idx..start_idx + c.len_utf8(),
                        slice: c.to_string(),
                    };
                    result.errors.push(LexerError::InvalidCharacter(c, span));
                }
            }
        }

        result
    }
}

impl AmmoniaLexer {
    fn lex_number(
        src: &str,
        start_idx: usize,
        number: String,
        end_idx: usize,
        result: &mut LexerResult,
    ) {
        let span = Span {
            range: start_idx..end_idx,
            slice: src[start_idx..end_idx].to_string(),
        };

        if number.contains('.') {
            if number.starts_with('.') {
                result.errors.push(LexerError::InvalidFloat(
                    InvalidFloatError::LeadingDecimal,
                    span,
                ));
            } else if number.ends_with('.') {
                result.errors.push(LexerError::InvalidFloat(
                    InvalidFloatError::TrailingDecimal,
                    span,
                ));
            } else {
                match number.parse::<f64>() {
                    Ok(f) if f.is_finite() => result.tokens.push(Token::Float(f, span)),
                    _ => result
                        .errors
                        .push(LexerError::InvalidFloat(InvalidFloatError::Overflow, span)),
                }
            }
        } else {
            match number.parse::<i64>() {
                Ok(i) => result.tokens.push(Token::Int(i, span)),
                Err(_) => {
                    result
                        .errors
                        .push(LexerError::InvalidInt(InvalidIntError::Overflow, span));
                }
            }
        }
    }
}
