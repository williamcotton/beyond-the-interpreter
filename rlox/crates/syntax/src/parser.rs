use rlox_text::{TextRange, TextSize};

use crate::expr::Expr;
use crate::{SyntaxKind, Token, scan};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub range: TextRange,
}

fn error_at(token: Token, message: &str) -> ParseError {
    let message = if token.kind == SyntaxKind::Error {
        "invalid token"
    } else {
        message
    };
    ParseError {
        message: message.to_owned(),
        range: token.range,
    }
}

pub fn parse(source: &str) -> Result<Expr, ParseError> {
    let mut tokens = scan(source)
        .into_iter()
        .filter(|token| !matches!(token.kind, SyntaxKind::Whitespace | SyntaxKind::LineComment));

    let token = match tokens.next() {
        Some(token) => token,
        None => {
            let end = TextSize(source.len());
            return Err(ParseError {
                message: "expected a number".to_owned(),
                range: TextRange::new(end, end).expect("ordered EOF range"),
            });
        }
    };
    if token.kind != SyntaxKind::Number {
        return Err(error_at(token, "expected a number"));
    }

    let spelling = token.range.slice(source).expect("scanner ranges are valid");
    let value = spelling
        .parse::<f64>()
        .map_err(|_| error_at(token, "invalid number"))?;

    if let Some(extra) = tokens.next() {
        return Err(error_at(extra, "expected end of input"));
    }

    Ok(Expr::Number(value))
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_number_spellings_and_surrounding_trivia() {
        for (source, expected) in [
            ("0", "0"),
            ("001.50", "1.5"),
            (" \t123 // note\n", "123"),
            (" // é\n12.5 // end", "12.5"),
        ] {
            let expr = parse(source).expect("one number");
            assert_eq!(expr.to_prefix(), expected, "{source:?}");
        }
    }

    #[test]
    fn missing_number_points_to_source_end() {
        for source in ["", " \t\n", " // é\n"] {
            let error = parse(source).unwrap_err();
            assert_eq!(error.message, "expected a number");
            assert_eq!(error.range.start().0, source.len());
            assert_eq!(error.range.end().0, source.len());
        }
    }

    #[test]
    fn rejects_a_second_number() {
        let error = parse("123 456").unwrap_err();
        assert_eq!(error.message, "expected end of input");
        assert_eq!(error.range.start().0, 4);
        assert_eq!(error.range.end().0, 7);
    }

    #[test]
    fn rejects_an_unexpected_token_kind() {
        let error = parse("hello").unwrap_err();
        assert_eq!(error.message, "expected a number");
        assert_eq!(error.range.start().0, 0);
        assert_eq!(error.range.end().0, 5);
    }

    #[test]
    fn retains_lexical_errors_and_utf8_ranges() {
        for (source, start, end) in [("é", 0, 2), ("123 é", 4, 6)] {
            let error = parse(source).unwrap_err();
            assert_eq!(error.message, "invalid token");
            assert_eq!(error.range.start().0, start);
            assert_eq!(error.range.end().0, end);
        }
    }
}
