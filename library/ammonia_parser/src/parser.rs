use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

use crate::{expr::Expr, token::Token};

pub struct AmmoniaParser {}
impl AmmoniaParser {
    fn parser<'tokens, 'src: 'tokens, I>()
    -> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token<'src>>>>
    where
        I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
    {
        recursive(|expr| {
            let atom = select! {
                Token::Float(n) => n,
            }
            .map(|n: &str| match n.parse::<f64>() {
                Ok(v) => Expr::Float(v),
                Err(_) => Expr::Error(format!("Could not parse {n} as f64")),
            });

            let block = expr
                .repeated()
                .collect()
                .map(Expr::Block)
                .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

            atom.or(block)
        })
    }
}

impl AmmoniaParser {
    #[must_use]
    pub fn from_src(src: &'_ str) -> ParseResult<Expr, Rich<'_, Token<'_>>> {
        let parser = Self::parser();

        let lexer = Token::lexer(src)
            .spanned()
            .map(|(token, span)| match token {
                Ok(token) => (token, span.into()),
                Err(()) => (Token::Error, span.into()),
            });

        let token_stream =
            Stream::from_iter(lexer).map((0..src.len()).into(), |(t, s): (_, _)| (t, s));

        parser.parse(token_stream)
    }
}
