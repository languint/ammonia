#[cfg(test)]
mod tests {
    use ammonia_parser::{expr::Expr, parser::AmmoniaParser};

    #[test]
    fn positive_no_decimal() {
        let src = "1";
        let result = AmmoniaParser::from_src(src);

        assert!(result.has_output());
        assert_eq!(result.into_output(), Some(Expr::Float(1.0)))
    }

    #[test]
    fn negative_no_decimal() {
        let src = "-1";
        let result = AmmoniaParser::from_src(src);

        assert!(result.has_output());
        assert_eq!(result.into_output(), Some(Expr::Float(-1.0)))
    }

    #[test]
    fn positive_with_decimal() {
        let src = "1.5";
        let result = AmmoniaParser::from_src(src);

        assert!(result.has_output());
        assert_eq!(result.into_output(), Some(Expr::Float(1.5)))
    }

    #[test]
    fn negative_with_decimal() {
        let src = "-1.5";
        let result = AmmoniaParser::from_src(src);

        assert!(result.has_output());
        assert_eq!(result.into_output(), Some(Expr::Float(-1.5)))
    }

    #[test]
    fn malformed_leading_dot() {
        let result = AmmoniaParser::from_src(".5");
        assert!(result.has_errors());
    }

    #[test]
    fn malformed_trailing_dot() {
        let result = AmmoniaParser::from_src("1.");
        assert!(result.has_errors());
    }
}
