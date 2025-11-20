pub mod defs;
pub mod error;
pub mod result;
pub mod token;

use token::Token;

macro_rules! single_char_matches {
    ($char:expr, $result:ident, $start_idx:ident, {
        $($ch:literal => $variant:ident),+ $(,)?
    }) => {{
        match $char {
            $(
                $ch => {
                    let span = Span {
                        range: $start_idx..$start_idx + 1,
                        slice: $ch.to_string(),
                    };
                    $result.tokens.push(Token::$variant(span));
                    true
                }
            )+
            _ => false,
        }
    }};
}

macro_rules! multi_char_matches {
    ($chars:ident, $result:ident, $start:ident, $src:expr, {
        $($lit:literal => $tok:ident),+ $(,)?
    }) => {{
        let mut matched = false;

        $(
            if !matched {
                let s = $lit;
                let mut ok = true;

                let mut clone = $chars.clone();

                for expected in s.chars().skip(1) {
                    match clone.next() {
                        Some((_, ch)) if ch == expected => {}
                        _ => { ok = false; break; }
                    }
                }

                if ok {

                    clone = $chars.clone();
                    for _ in s.chars().skip(1) {
                        $chars.next();
                    }

                    let end = clone
                        .take(s.len())
                        .last()
                        .map(|(i, c)| i + c.len_utf8())
                        .unwrap_or($start + s.len());

                    let span = Span {
                        range: $start..end,
                        slice: $src[$start..end].to_string(),
                    };
                    $result.tokens.push(Token::$tok(span));
                    matched = true;
                }
            }
        )+

        matched
    }};
}

use crate::lexer::{
    defs::Span,
    error::{InvalidFloatError, InvalidIntError, LexerError},
    result::LexerResult,
};

pub struct AmmoniaLexer {
    pub src: String,
}
impl AmmoniaLexer {
    pub fn from_src(src: String) -> Self {
        Self { src }
    }
}

impl AmmoniaLexer {
    pub fn lex(&self) -> LexerResult {
        let mut result = LexerResult::default();
        let mut chars = self.src.char_indices().peekable();

        while let Some((start_idx, c)) = chars.next() {
            if multi_char_matches!(chars, result, start_idx, self.src, {
                "==" => EqualEqual,
                "!=" => BangEqual,
                "<=" => LessEqual,
                ">=" => GreaterEqual,
                "->" => Arrow,
                "=>" => FatArrow,
            }) {
                continue;
            }

            if single_char_matches!(c, result, start_idx, {
                '{' => LeftBrace,
                '}' => RightBrace,
                '(' => LeftParen,
                ')' => RightParen,
                '+' => Plus,
                '*' => Star,
                '/' => Slash,
                '\\' => Backslash,
                '=' => Equal,
            }) {
                continue;
            }

            match c {
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

                    result.tokens.push(Token::Minus(Span {
                        range: start_idx..start_idx + 1,
                        slice: "-".to_string(),
                    }))
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
