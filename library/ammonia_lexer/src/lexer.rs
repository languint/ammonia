pub mod error;
pub mod result;
pub mod token;

use token::Token;

use ammonia_defs::source::Span;

macro_rules! single_char_matches {
    ($char:expr, $result:ident, $start_idx:ident, $source_id:expr, {
        $($ch:literal => $variant:ident),+ $(,)?
    }) => {{
        match $char {
            $(
                $ch => {
                    let span = Span {
                        range: $start_idx..$start_idx + 1,
                        slice: $ch.to_string(),
                        source_id: $source_id,
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
    ($chars:ident, $result:ident, $start:ident, $src:expr, $source_id:expr, {
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
                        source_id: $source_id,
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
    error::{InvalidFloatError, InvalidIntError, LexerError},
    result::LexerResult,
};

pub struct AmmoniaLexer {
    pub src: String,
    pub source_id: String,
}
impl AmmoniaLexer {
    #[must_use]
    pub fn from_src(src: String, source_id: String) -> Self {
        Self { src, source_id }
    }
}

impl AmmoniaLexer {
    #[must_use]
    pub fn lex(&self) -> LexerResult {
        let mut result = LexerResult::default();
        let mut chars = self.src.char_indices().peekable();

        while let Some((start_idx, c)) = chars.next() {
            if multi_char_matches!(chars, result, start_idx, self.src, self.source_id.clone(), {
                "==" => EqualEqual,
                "!=" => BangEqual,
                "<=" => LessEqual,
                ">=" => GreaterEqual,
                "->" => Arrow,
                "=>" => FatArrow,
            }) {
                continue;
            }

            if single_char_matches!(c, result, start_idx, self.source_id.clone(), {
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
                    if let Some(&(_, next_c)) = chars.peek()
                        && (next_c.is_ascii_digit() || next_c == '.')
                    {
                        let mut number = "-".to_string();
                        let mut end_idx = start_idx + 1;
                        while let Some(&(idx, next_c)) = chars.peek() {
                            if next_c.is_ascii_digit() || next_c == '.' {
                                number.push(next_c);
                                end_idx = idx + 1;
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        self.lex_number(start_idx, &number, end_idx, &mut result);
                        continue;
                    }

                    result.tokens.push(Token::Minus(Span {
                        range: start_idx..start_idx + 1,
                        slice: "-".to_string(),
                        source_id: self.source_id.clone(),
                    }));
                }
                '0'..='9' | '.' => {
                    let mut number = c.to_string();
                    let mut end_idx = start_idx + 1;
                    while let Some(&(idx, next_c)) = chars.peek() {
                        if next_c.is_ascii_digit() || next_c == '.' {
                            number.push(next_c);
                            end_idx = idx + 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    self.lex_number(start_idx, &number, end_idx, &mut result);
                }
                c if c.is_whitespace() => {}
                _ => {
                    let span = Span {
                        range: start_idx..start_idx + c.len_utf8(),
                        slice: c.to_string(),
                        source_id: self.source_id.clone(),
                    };
                    result.errors.push(LexerError::InvalidCharacter(c, span));
                }
            }
        }

        result
    }
}

impl AmmoniaLexer {
    fn lex_number(&self, start_idx: usize, number: &str, end_idx: usize, result: &mut LexerResult) {
        let span = Span {
            range: start_idx..end_idx,
            slice: self.src[start_idx..end_idx].to_string(),
            source_id: self.source_id.clone(),
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
