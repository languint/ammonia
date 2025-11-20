#[cfg(test)]
mod tests {
    use ammonia_lexer::lexer::{
        AmmoniaLexer,
        defs::Span,
        error::{InvalidFloatError, InvalidIntError, LexerError},
        token::Token,
    };

    #[test]
    fn positive_float() {
        let src = "1.0";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(!result.has_errors());
        assert_eq!(
            result.tokens,
            vec![Token::Float(
                1.0,
                Span {
                    range: 0..3,
                    slice: "1.0".into()
                }
            )]
        );
    }

    #[test]
    fn negative_float() {
        let src = "-1.0";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(!result.has_errors());
        assert_eq!(
            result.tokens,
            vec![Token::Float(
                -1.0,
                Span {
                    range: 0..4,
                    slice: "-1.0".into()
                }
            )]
        );
    }

    #[test]
    fn trailing_decimal() {
        let src = "1.";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(result.has_errors());
        assert_eq!(
            result.errors,
            vec![LexerError::InvalidFloat(
                InvalidFloatError::TrailingDecimal,
                Span {
                    range: 0..2,
                    slice: "1.".into()
                }
            )]
        );
    }

    #[test]
    fn leading_decimal() {
        let src = ".1";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(result.has_errors());
        assert_eq!(
            result.errors,
            vec![LexerError::InvalidFloat(
                InvalidFloatError::LeadingDecimal,
                Span {
                    range: 0..2,
                    slice: ".1".into()
                }
            )]
        );
    }

    #[test]
    fn positive_int() {
        let src = "42";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(!result.has_errors());
        assert_eq!(
            result.tokens,
            vec![Token::Int(
                42,
                Span {
                    range: 0..2,
                    slice: "42".into()
                }
            )]
        );
    }

    #[test]
    fn negative_int() {
        let src = "-42";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(!result.has_errors());
        assert_eq!(
            result.tokens,
            vec![Token::Int(
                -42,
                Span {
                    range: 0..3,
                    slice: "-42".into()
                }
            )]
        );
    }

    #[test]
    fn int_overflow() {
        let src = "9223372036854775808";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(result.has_errors());
        assert_eq!(
            result.errors,
            vec![LexerError::InvalidInt(
                InvalidIntError::Overflow,
                Span {
                    range: 0..19,
                    slice: "9223372036854775808".into()
                }
            )]
        );
    }

    #[test]
    fn float_overflow_large() {
        let src = "1797693134862315708145274237317043567980705675258449965989174768031572607800285387605895586327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687381771809192998812504040261841248583680.0";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(result.has_errors());
        assert_eq!(
            result.errors,
            vec![LexerError::InvalidFloat(
                InvalidFloatError::Overflow,
                Span {
                    range: 0..312,
                    slice: src.into()
                }
            )]
        );
    }

    #[test]
    fn braces_and_whitespace() {
        let src = "{ 123 -456.78 }";
        let lexer = AmmoniaLexer::from_src(src.to_string());
        let result = lexer.parse();
        assert!(!result.has_errors());
        assert_eq!(
            result.tokens,
            vec![
                Token::LeftBrace(Span {
                    range: 0..1,
                    slice: "{".into()
                }),
                Token::Int(
                    123,
                    Span {
                        range: 2..5,
                        slice: "123".into()
                    }
                ),
                Token::Float(
                    -456.78,
                    Span {
                        range: 6..13,
                        slice: "-456.78".into()
                    }
                ),
                Token::RightBrace(Span {
                    range: 14..15,
                    slice: "}".into()
                }),
            ]
        );
    }
}
