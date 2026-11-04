use rlox_text::{TextRange, TextSize};

use crate::expr::Expr;
use crate::{SyntaxKind, Token, scan};

const MAX_NESTING: usize = 64;

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
    let mut parser = Parser::new(source);
    let expr = parser.expression(0)?;
    if parser.peek().is_some() {
        return Err(parser.error("expected end of input"));
    }
    Ok(expr)
}

struct Parser<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    current: usize,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        let mut tokens = scan(source);
        tokens.retain(|token| {
            !matches!(token.kind, SyntaxKind::Whitespace | SyntaxKind::LineComment)
        });
        Self {
            source,
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self, nesting: usize) -> Result<Expr, ParseError> {
        self.primary(nesting)
    }

    fn primary(&mut self, nesting: usize) -> Result<Expr, ParseError> {
        let token = match self.peek() {
            Some(token) => token,
            None => return Err(self.error("expected an expression")),
        };
        match token.kind {
            SyntaxKind::Number => self.number(),
            SyntaxKind::LeftParen => {
                if nesting >= MAX_NESTING {
                    return Err(self.error("expression nesting limit exceeded"));
                }
                self.bump();
                let inner = self.expression(nesting + 1)?;
                match self.peek() {
                    Some(close) if close.kind == SyntaxKind::RightParen => self.bump(),
                    _ => return Err(self.error("expected ')'")),
                }
                Ok(Expr::Grouping(Box::new(inner)))
            }
            _ => Err(self.error("expected an expression")),
        }
    }

    fn number(&mut self) -> Result<Expr, ParseError> {
        let token = match self.peek() {
            Some(token) => token,
            None => return Err(self.error("expected a number")),
        };
        if token.kind != SyntaxKind::Number {
            return Err(self.error("expected a number"));
        }

        let spelling = token
            .range
            .slice(self.source)
            .expect("scanner ranges are valid");
        let value = spelling
            .parse::<f64>()
            .map_err(|_| error_at(token, "invalid number"))?;
        self.bump();
        Ok(Expr::Number(value))
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).copied()
    }

    fn bump(&mut self) {
        self.current += 1;
    }

    fn error(&self, message: &str) -> ParseError {
        match self.peek() {
            Some(token) => error_at(token, message),
            None => {
                let end = TextSize(self.source.len());
                ParseError {
                    message: message.to_owned(),
                    range: TextRange::new(end, end).expect("ordered EOF range"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_NESTING, parse};

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
    fn missing_expression_points_to_source_end() {
        for source in ["", " \t\n", " // é\n"] {
            let error = parse(source).unwrap_err();
            assert_eq!(error.message, "expected an expression");
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
        assert_eq!(error.message, "expected an expression");
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

    #[test]
    fn parses_nested_groups_and_trivia() {
        for (source, expected) in [
            ("(123)", "(group 123)"),
            ("((12.5))", "(group (group 12.5))"),
            ("( // é\n123 )", "(group 123)"),
        ] {
            assert_eq!(parse(source).unwrap().to_prefix(), expected);
        }
    }

    #[test]
    fn reports_missing_closing_parentheses() {
        for (source, start, end) in [("(123", 4, 4), ("(123 456)", 5, 8), ("(123 // é\n", 11, 11)]
        {
            let error = parse(source).unwrap_err();
            assert_eq!(error.message, "expected ')'");
            assert_eq!(error.range.start().0, start);
            assert_eq!(error.range.end().0, end);
        }
    }

    #[test]
    fn rejects_empty_groups_and_trailing_closes() {
        let empty = parse("()").unwrap_err();
        assert_eq!(empty.message, "expected an expression");
        assert_eq!(empty.range.start().0, 1);
        assert_eq!(empty.range.end().0, 2);

        let extra = parse("123)").unwrap_err();
        assert_eq!(extra.message, "expected end of input");
        assert_eq!(extra.range.start().0, 3);
        assert_eq!(extra.range.end().0, 4);
    }

    #[test]
    fn retains_lexical_errors_inside_groups() {
        for (source, start, end) in [("(é)", 1, 3), ("(123 é)", 5, 7)] {
            let error = parse(source).unwrap_err();
            assert_eq!(error.message, "invalid token");
            assert_eq!(error.range.start().0, start);
            assert_eq!(error.range.end().0, end);
        }
    }

    #[test]
    fn checks_both_sides_of_the_nesting_limit() {
        let at_limit = format!("{}1{}", "(".repeat(MAX_NESTING), ")".repeat(MAX_NESTING));
        let expr = parse(&at_limit).expect("the configured depth is accepted");
        assert_eq!(
            expr.to_prefix(),
            format!(
                "{}1{}",
                "(group ".repeat(MAX_NESTING),
                ")".repeat(MAX_NESTING)
            )
        );

        let too_deep = format!("({at_limit})");
        let error = parse(&too_deep).unwrap_err();
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(error.range.start().0, MAX_NESTING);
        assert_eq!(error.range.end().0, MAX_NESTING + 1);
    }
}
