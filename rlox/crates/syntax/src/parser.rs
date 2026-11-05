use rlox_text::{SourceTooLarge, TextRange, checked_range, checked_size, slice};

use crate::expr::{BinaryOp, Expr, UnaryOp};
use crate::{SyntaxKind, Token, scan};

const MAX_NESTING: usize = 64;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub range: TextRange,
}

#[derive(Debug)]
pub enum ParseFailure {
    SourceTooLarge(SourceTooLarge),
    Syntax(ParseError),
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

fn tree_height(expr: &Expr) -> usize {
    match expr {
        Expr::Number(_) => 0,
        Expr::Grouping(child) => 1 + tree_height(child),
        Expr::Unary { right, .. } => 1 + tree_height(right),
        Expr::Binary { left, right, .. } => 1 + tree_height(left).max(tree_height(right)),
    }
}

pub fn parse(source: &str) -> Result<Expr, ParseFailure> {
    let mut parser = Parser::new(source).map_err(ParseFailure::SourceTooLarge)?;
    let expr = parser.expression(0).map_err(ParseFailure::Syntax)?;
    if parser.peek().is_some() {
        return Err(ParseFailure::Syntax(parser.error("expected end of input")));
    }
    Ok(expr)
}

struct Parser<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    current: usize,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Result<Self, SourceTooLarge> {
        let mut tokens = scan(source)?;
        tokens.retain(|token| {
            !matches!(token.kind, SyntaxKind::Whitespace | SyntaxKind::LineComment)
        });
        Ok(Self {
            source,
            tokens,
            current: 0,
        })
    }

    fn expression(&mut self, nesting: usize) -> Result<Expr, ParseError> {
        self.term(nesting)
    }

    fn term(&mut self, nesting: usize) -> Result<Expr, ParseError> {
        let mut expr = self.factor(nesting)?;
        let mut height = tree_height(&expr);

        while let Some(token) = self.peek() {
            let op = match token.kind {
                SyntaxKind::Plus => BinaryOp::Add,
                SyntaxKind::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.bump();
            let right = self.factor(nesting)?;
            let next_height = 1 + height.max(tree_height(&right));
            if nesting + next_height > MAX_NESTING {
                return Err(error_at(token, "expression nesting limit exceeded"));
            }
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
            height = next_height;
        }

        Ok(expr)
    }

    fn factor(&mut self, nesting: usize) -> Result<Expr, ParseError> {
        let mut expr = self.unary(nesting)?;
        let mut height = tree_height(&expr);

        while let Some(token) = self.peek() {
            let op = match token.kind {
                SyntaxKind::Star => BinaryOp::Multiply,
                SyntaxKind::Slash => BinaryOp::Divide,
                _ => break,
            };
            self.bump();
            let right = self.unary(nesting)?;
            let next_height = 1 + height.max(tree_height(&right));
            if nesting + next_height > MAX_NESTING {
                return Err(error_at(token, "expression nesting limit exceeded"));
            }
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
            height = next_height;
        }

        Ok(expr)
    }

    fn unary(&mut self, nesting: usize) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(token) if token.kind == SyntaxKind::Minus => {
                if nesting >= MAX_NESTING {
                    return Err(self.error("expression nesting limit exceeded"));
                }
                self.bump();
                let right = self.unary(nesting + 1)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    right: Box::new(right),
                })
            }
            _ => self.primary(nesting),
        }
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

        let spelling = slice(self.source, token.range).expect("scanner ranges are valid");
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
                let end = checked_size(self.source.len()).expect("validated source length");
                ParseError {
                    message: message.to_owned(),
                    range: checked_range(end, end).expect("ordered EOF range"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_NESTING, ParseError, ParseFailure, parse};

    fn syntax_error(source: &str) -> ParseError {
        match parse(source) {
            Err(ParseFailure::Syntax(error)) => error,
            other => panic!("expected a syntax error, got {other:?}"),
        }
    }

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
            let error = syntax_error(source);
            assert_eq!(error.message, "expected an expression");
            assert_eq!(usize::from(error.range.start()), source.len());
            assert_eq!(usize::from(error.range.end()), source.len());
        }
    }

    #[test]
    fn rejects_a_second_number() {
        let error = syntax_error("123 456");
        assert_eq!(error.message, "expected end of input");
        assert_eq!(usize::from(error.range.start()), 4);
        assert_eq!(usize::from(error.range.end()), 7);
    }

    #[test]
    fn rejects_an_unexpected_token_kind() {
        let error = syntax_error("hello");
        assert_eq!(error.message, "expected an expression");
        assert_eq!(usize::from(error.range.start()), 0);
        assert_eq!(usize::from(error.range.end()), 5);
    }

    #[test]
    fn retains_lexical_errors_and_utf8_ranges() {
        for (source, start, end) in [("é", 0, 2), ("123 é", 4, 6)] {
            let error = syntax_error(source);
            assert_eq!(error.message, "invalid token");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
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
            let error = syntax_error(source);
            assert_eq!(error.message, "expected ')'");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
        }
    }

    #[test]
    fn rejects_empty_groups_and_trailing_closes() {
        let empty = syntax_error("()");
        assert_eq!(empty.message, "expected an expression");
        assert_eq!(usize::from(empty.range.start()), 1);
        assert_eq!(usize::from(empty.range.end()), 2);

        let extra = syntax_error("123)");
        assert_eq!(extra.message, "expected end of input");
        assert_eq!(usize::from(extra.range.start()), 3);
        assert_eq!(usize::from(extra.range.end()), 4);
    }

    #[test]
    fn retains_lexical_errors_inside_groups() {
        for (source, start, end) in [("(é)", 1, 3), ("(123 é)", 5, 7)] {
            let error = syntax_error(source);
            assert_eq!(error.message, "invalid token");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
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
        let error = syntax_error(&too_deep);
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(usize::from(error.range.start()), MAX_NESTING);
        assert_eq!(usize::from(error.range.end()), MAX_NESTING + 1);
    }

    #[test]
    fn parses_unary_chains_grouping_and_trivia() {
        for (source, expected) in [
            ("-123", "(- 123)"),
            ("--123", "(- (- 123))"),
            ("-(123)", "(- (group 123))"),
            ("(-123)", "(group (- 123))"),
            (" - // é\n-12.5 ", "(- (- 12.5))"),
        ] {
            assert_eq!(parse(source).unwrap().to_prefix(), expected, "{source:?}");
        }
    }

    #[test]
    fn reports_unary_operand_and_delimiter_errors() {
        for (source, message, start, end) in [
            ("-", "expected an expression", 1, 1),
            ("--", "expected an expression", 2, 2),
            ("(-)", "expected an expression", 2, 3),
            ("-é", "invalid token", 1, 3),
            ("-123)", "expected end of input", 4, 5),
            ("(-123", "expected ')'", 5, 5),
        ] {
            let error = syntax_error(source);
            assert_eq!(error.message, message, "{source:?}");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
        }
    }

    #[test]
    fn bounds_unary_nesting() {
        let at_limit = format!("{}1", "-".repeat(MAX_NESTING));
        let expr = parse(&at_limit).expect("the configured unary depth is accepted");
        assert_eq!(
            expr.to_prefix(),
            format!("{}1{}", "(- ".repeat(MAX_NESTING), ")".repeat(MAX_NESTING))
        );

        let error = syntax_error(&format!("-{at_limit}"));
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(usize::from(error.range.start()), MAX_NESTING);
        assert_eq!(usize::from(error.range.end()), MAX_NESTING + 1);
    }

    #[test]
    fn shares_the_budget_between_groups_and_negation() {
        let groups = MAX_NESTING / 2;
        let signs = MAX_NESTING - groups;
        let at_limit = format!(
            "{}{}1{}",
            "(".repeat(groups),
            "-".repeat(signs),
            ")".repeat(groups)
        );
        let expr = parse(&at_limit).expect("the combined depth is accepted");
        assert_eq!(
            expr.to_prefix(),
            format!(
                "{}{}1{}",
                "(group ".repeat(groups),
                "(- ".repeat(signs),
                ")".repeat(MAX_NESTING)
            )
        );

        let too_deep = format!(
            "{}{}1{}",
            "(".repeat(groups),
            "-".repeat(signs + 1),
            ")".repeat(groups)
        );
        let error = syntax_error(&too_deep);
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(usize::from(error.range.start()), MAX_NESTING);
        assert_eq!(usize::from(error.range.end()), MAX_NESTING + 1);
    }

    #[test]
    fn parses_multiplicative_chains_and_unary_operands() {
        for (source, expected) in [
            ("8 / 4 / 2", "(/ (/ 8 4) 2)"),
            ("8 * 4 / 2", "(/ (* 8 4) 2)"),
            ("8 / (4 / 2)", "(/ 8 (group (/ 4 2)))"),
            ("-8 * 4 / 2", "(/ (* (- 8) 4) 2)"),
            ("8 * // é\n2", "(* 8 2)"),
            ("8 / 0", "(/ 8 0)"),
        ] {
            assert_eq!(parse(source).unwrap().to_prefix(), expected, "{source:?}");
        }
    }

    #[test]
    fn reports_multiplicative_operand_errors() {
        for (source, message, start, end) in [
            ("8 *", "expected an expression", 3, 3),
            ("8 **2", "expected an expression", 3, 4),
            ("8 / é", "invalid token", 4, 6),
            ("(8 /)", "expected an expression", 4, 5),
            ("8 / 2 3", "expected end of input", 6, 7),
        ] {
            let error = syntax_error(source);
            assert_eq!(error.message, message, "{source:?}");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
        }
    }

    #[test]
    fn bounds_the_height_of_a_left_fold() {
        let at_limit = format!("1{}", " * 1".repeat(MAX_NESTING));
        let expr = parse(&at_limit).expect("the configured binary height is accepted");
        assert_eq!(
            expr.to_prefix(),
            format!(
                "{}1{}",
                "(* ".repeat(MAX_NESTING),
                " 1)".repeat(MAX_NESTING)
            )
        );

        let error = syntax_error(&format!("{at_limit} * 1"));
        let operator = 2 + 4 * MAX_NESTING;
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(usize::from(error.range.start()), operator);
        assert_eq!(usize::from(error.range.end()), operator + 1);
    }

    #[test]
    fn includes_both_operand_heights() {
        let signs = "-".repeat(MAX_NESTING - 1);
        for source in [format!("{signs}1 * 1"), format!("1 * {signs}1")] {
            assert!(parse(&source).is_ok(), "{source}");
        }

        let signs = "-".repeat(MAX_NESTING);
        for (source, operator) in [
            (format!("{signs}1 * 1"), MAX_NESTING + 2),
            (format!("1 * {signs}1"), 2),
        ] {
            let error = syntax_error(&source);
            assert_eq!(error.message, "expression nesting limit exceeded");
            assert_eq!(usize::from(error.range.start()), operator);
            assert_eq!(usize::from(error.range.end()), operator + 1);
        }
    }

    #[test]
    fn includes_enclosing_grouping_in_the_binary_budget() {
        let at_limit = format!("(1{})", " * 1".repeat(MAX_NESTING - 1));
        assert!(parse(&at_limit).is_ok());

        let too_deep = format!("(1{})", " * 1".repeat(MAX_NESTING));
        let error = syntax_error(&too_deep);
        let operator = 3 + 4 * (MAX_NESTING - 1);
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(usize::from(error.range.start()), operator);
        assert_eq!(usize::from(error.range.end()), operator + 1);
    }

    #[test]
    fn parses_additive_precedence_associativity_and_unary_operands() {
        for (source, expected) in [
            ("1 + 2 * 3", "(+ 1 (* 2 3))"),
            ("(1 + 2) * 3", "(* (group (+ 1 2)) 3)"),
            ("8 - 3 - 1", "(- (- 8 3) 1)"),
            ("8 + 3 - 1", "(- (+ 8 3) 1)"),
            ("1--2", "(- 1 (- 2))"),
            ("1+-2", "(+ 1 (- 2))"),
            ("-(1 + 2) * 3", "(* (- (group (+ 1 2))) 3)"),
            ("1 + 2 * 3 - 4 / 2", "(- (+ 1 (* 2 3)) (/ 4 2))"),
        ] {
            assert_eq!(parse(source).unwrap().to_prefix(), expected, "{source:?}");
        }
    }

    #[test]
    fn reports_additive_operand_and_grouping_errors() {
        for (source, message, start, end) in [
            ("1 +", "expected an expression", 3, 3),
            ("1 + +2", "expected an expression", 4, 5),
            ("1 + é", "invalid token", 4, 6),
            ("(1 + 2", "expected ')'", 6, 6),
            ("1 + 2 3", "expected end of input", 6, 7),
            ("1 + (2 * )", "expected an expression", 9, 10),
        ] {
            let error = syntax_error(source);
            assert_eq!(error.message, message, "{source:?}");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
        }
    }

    #[test]
    fn numeric_spelling_and_trivia_do_not_survive_in_the_ast() {
        for source in ["1 + 2 * 3", " 001.0 // first\n + 02.00 * 3"] {
            assert_eq!(parse(source).unwrap().to_prefix(), "(+ 1 (* 2 3))");
        }
    }

    #[test]
    fn bounds_the_height_of_an_additive_chain() {
        let at_limit = format!("1{}", " + 1".repeat(MAX_NESTING));
        let expr = parse(&at_limit).expect("the configured additive height is accepted");
        assert_eq!(
            expr.to_prefix(),
            format!(
                "{}1{}",
                "(+ ".repeat(MAX_NESTING),
                " 1)".repeat(MAX_NESTING)
            )
        );

        let error = syntax_error(&format!("{at_limit} + 1"));
        let operator = 2 + 4 * MAX_NESTING;
        assert_eq!(error.message, "expression nesting limit exceeded");
        assert_eq!(usize::from(error.range.start()), operator);
        assert_eq!(usize::from(error.range.end()), operator + 1);
    }

    #[test]
    fn includes_factors_and_enclosing_groups_in_the_sum_budget() {
        let factors = " * 1".repeat(MAX_NESTING - 1);
        let sums = " + 1".repeat(MAX_NESTING - 1);
        for source in [
            format!("1{factors} + 1"),
            format!("1 + 1{factors}"),
            format!("(1{sums})"),
        ] {
            assert!(parse(&source).is_ok(), "{source}");
        }

        let factors = " * 1".repeat(MAX_NESTING);
        let sums = " + 1".repeat(MAX_NESTING);
        for (source, operator) in [
            (format!("1{factors} + 1"), 2 + 4 * MAX_NESTING),
            (format!("1 + 1{factors}"), 2),
            (format!("(1{sums})"), 3 + 4 * (MAX_NESTING - 1)),
        ] {
            let error = syntax_error(&source);
            assert_eq!(error.message, "expression nesting limit exceeded");
            assert_eq!(usize::from(error.range.start()), operator);
            assert_eq!(usize::from(error.range.end()), operator + 1);
        }
    }
}
