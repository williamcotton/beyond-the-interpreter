use rlox_text::{SourceTooLarge, checked_range, checked_size, slice};
use rowan::GreenNodeBuilder;

use crate::parser::ParseError;
use crate::tree::SyntaxNode;
use crate::{SyntaxKind, Token, scan};

const MAX_NESTING: usize = 64;

#[derive(Debug)]
pub struct Parse {
    pub root: SyntaxNode,
    pub errors: Vec<ParseError>,
}

pub fn parse(source: &str) -> Result<Parse, SourceTooLarge> {
    let mut parser = Parser::new(source)?;
    parser.builder.start_node(SyntaxKind::Root.into_raw());
    if let Err(error) = parser.expression(0) {
        parser.errors.push(error);
        while parser.current < parser.tokens.len() {
            parser.bump();
        }
    } else {
        parser.eat_trivia();
        while parser.peek().is_some() {
            let error = parser.error("expected end of input");
            parser.errors.push(error);
            parser.bump();
            parser.eat_trivia();
        }
    }
    parser.builder.finish_node();
    let root = SyntaxNode::new_root(parser.builder.finish());
    Ok(Parse {
        root,
        errors: parser.errors,
    })
}

struct Parser<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
    builder: GreenNodeBuilder<'static>,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Result<Self, SourceTooLarge> {
        Ok(Self {
            source,
            tokens: scan(source)?,
            current: 0,
            errors: Vec::new(),
            builder: GreenNodeBuilder::new(),
        })
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).copied()
    }

    fn peek_non_trivia(&self) -> Option<Token> {
        self.tokens[self.current..]
            .iter()
            .copied()
            .find(|token| !matches!(token.kind, SyntaxKind::Whitespace | SyntaxKind::LineComment))
    }

    fn bump(&mut self) {
        let token = self.tokens[self.current];
        let spelling = slice(self.source, token.range).expect("scanner ranges are valid");
        self.builder.token(token.kind.into_raw(), spelling);
        self.current += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.peek() {
            if !matches!(token.kind, SyntaxKind::Whitespace | SyntaxKind::LineComment) {
                break;
            }
            self.bump();
        }
    }

    fn error(&self, message: &str) -> ParseError {
        match self.peek() {
            Some(token) => ParseError {
                message: if token.kind == SyntaxKind::Error {
                    "invalid token".to_owned()
                } else {
                    message.to_owned()
                },
                range: token.range,
            },
            None => {
                let end = checked_size(self.source.len()).expect("validated source length");
                ParseError {
                    message: message.to_owned(),
                    range: checked_range(end, end).expect("ordered EOF range"),
                }
            }
        }
    }

    fn expression(&mut self, nesting: usize) -> Result<usize, ParseError> {
        self.term(nesting)
    }

    fn term(&mut self, nesting: usize) -> Result<usize, ParseError> {
        self.eat_trivia();
        let checkpoint = self.builder.checkpoint();
        let mut height = self.factor(nesting)?;

        while let Some(operator) = self.peek_non_trivia() {
            if !matches!(operator.kind, SyntaxKind::Plus | SyntaxKind::Minus) {
                break;
            }
            self.eat_trivia();
            self.bump();
            let right_height = self.factor(nesting)?;
            let next_height = 1 + height.max(right_height);
            if nesting + next_height > MAX_NESTING {
                return Err(ParseError {
                    message: "expression nesting limit exceeded".to_owned(),
                    range: operator.range,
                });
            }
            self.builder
                .start_node_at(checkpoint, SyntaxKind::BinaryExpr.into_raw());
            self.builder.finish_node();
            height = next_height;
        }

        Ok(height)
    }

    fn factor(&mut self, nesting: usize) -> Result<usize, ParseError> {
        self.eat_trivia();
        let checkpoint = self.builder.checkpoint();
        let mut height = self.unary(nesting)?;

        while let Some(operator) = self.peek_non_trivia() {
            if !matches!(operator.kind, SyntaxKind::Star | SyntaxKind::Slash) {
                break;
            }
            self.eat_trivia();
            self.bump();
            let right_height = self.unary(nesting)?;
            let next_height = 1 + height.max(right_height);
            if nesting + next_height > MAX_NESTING {
                return Err(ParseError {
                    message: "expression nesting limit exceeded".to_owned(),
                    range: operator.range,
                });
            }
            self.builder
                .start_node_at(checkpoint, SyntaxKind::BinaryExpr.into_raw());
            self.builder.finish_node();
            height = next_height;
        }

        Ok(height)
    }

    fn missing_expression(&mut self) {
        self.builder.start_node(SyntaxKind::MissingExpr.into_raw());
        self.builder.finish_node();
    }

    fn unary(&mut self, nesting: usize) -> Result<usize, ParseError> {
        let mut reported = false;
        loop {
            self.eat_trivia();
            match self.peek() {
                Some(token) if token.kind == SyntaxKind::Minus => {
                    if nesting >= MAX_NESTING {
                        return Err(self.error("expression nesting limit exceeded"));
                    }
                    self.builder.start_node(SyntaxKind::UnaryExpr.into_raw());
                    self.bump();
                    let result = self.unary(nesting + 1);
                    self.builder.finish_node();
                    return result.map(|height| height + 1);
                }
                Some(token) if matches!(token.kind, SyntaxKind::Number | SyntaxKind::LeftParen) => {
                    return self.primary(nesting);
                }
                None
                | Some(Token {
                    kind:
                        SyntaxKind::Plus | SyntaxKind::Star | SyntaxKind::Slash | SyntaxKind::RightParen,
                    ..
                }) => {
                    if !reported {
                        let error = self.error("expected an expression");
                        self.errors.push(error);
                    }
                    self.missing_expression();
                    return Ok(0);
                }
                Some(_) => {
                    let error = self.error("expected an expression");
                    self.errors.push(error);
                    self.bump();
                    reported = true;
                }
            }
        }
    }

    fn primary(&mut self, nesting: usize) -> Result<usize, ParseError> {
        match self.peek() {
            Some(token) if token.kind == SyntaxKind::Number => {
                self.builder.start_node(SyntaxKind::NumberExpr.into_raw());
                self.bump();
                self.builder.finish_node();
                Ok(0)
            }
            Some(token) if token.kind == SyntaxKind::LeftParen => self.group(nesting),
            _ => unreachable!("unary checks primary starters"),
        }
    }

    fn close_group(&mut self) {
        loop {
            self.eat_trivia();
            match self.peek() {
                Some(token) if token.kind == SyntaxKind::RightParen => {
                    self.bump();
                    break;
                }
                Some(_) => {
                    let error = self.error("expected ')'");
                    self.errors.push(error);
                    self.bump();
                }
                None => {
                    let error = self.error("expected ')'");
                    self.errors.push(error);
                    break;
                }
            }
        }
    }

    fn group(&mut self, nesting: usize) -> Result<usize, ParseError> {
        if nesting >= MAX_NESTING {
            return Err(self.error("expression nesting limit exceeded"));
        }
        self.builder.start_node(SyntaxKind::GroupingExpr.into_raw());
        self.bump();
        let result = match self.expression(nesting + 1) {
            Err(error) => Err(error),
            Ok(inner_height) => {
                self.close_group();
                Ok(inner_height + 1)
            }
        };
        self.builder.finish_node();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_NESTING, parse};
    use crate::SyntaxKind;
    use crate::tree::SyntaxNode;

    #[test]
    fn groups_numbers_and_owns_trivia_at_the_container() {
        let source = " ( 001.50 ) ";
        let parsed = parse(source).expect("small source");
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.root.to_string(), source);
        let group = parsed.root.first_child().expect("group");
        assert_eq!(group.kind(), SyntaxKind::GroupingExpr);
        assert_eq!(usize::from(group.text_range().start()), 1);
        assert_eq!(usize::from(group.text_range().end()), 11);
        assert_eq!(group.to_string(), "( 001.50 )");
        let number = group.first_child().expect("number");
        assert_eq!(number.kind(), SyntaxKind::NumberExpr);
        assert_eq!(number.parent(), Some(group.clone()));
        assert_eq!(number.to_string(), "001.50");
        assert_eq!(usize::from(number.text_range().start()), 3);
        assert_eq!(usize::from(number.text_range().end()), 9);
        let kinds: Vec<_> = group
            .children_with_tokens()
            .map(|child| child.kind())
            .collect();
        assert_eq!(
            kinds,
            vec![
                SyntaxKind::LeftParen,
                SyntaxKind::Whitespace,
                SyntaxKind::NumberExpr,
                SyntaxKind::Whitespace,
                SyntaxKind::RightParen
            ]
        );
        assert_eq!(parsed.root.children_with_tokens().count(), 3);
    }

    #[test]
    fn preserves_source_during_local_recovery() {
        for source in [
            "",
            " // é\n",
            "001.50",
            "((12.5))",
            " // é\n(12.5 // end",
            "()",
            "(1 @)",
            "é",
            "1 2",
            "1 + 2",
            "((1) extra)",
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.root.kind(), SyntaxKind::Root);
            assert_eq!(parsed.root.to_string(), source);
            assert_eq!(usize::from(parsed.root.text_range().start()), 0);
            assert_eq!(usize::from(parsed.root.text_range().end()), source.len());
        }
        for source in ["001.50", "((12.5))", " // é\n(12.5) // end"] {
            assert!(parse(source).expect("small source").errors.is_empty());
        }
    }

    #[test]
    fn reports_located_errors_and_recovers_group_closers() {
        for (source, message, start, end) in [
            ("", "expected an expression", 0, 0),
            (" // é\n", "expected an expression", 7, 7),
            ("(12.5", "expected ')'", 5, 5),
            ("()", "expected an expression", 1, 2),
            ("(1 @)", "invalid token", 3, 4),
            ("é", "invalid token", 0, 2),
            ("1 2", "expected end of input", 2, 3),
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.errors.len(), 1, "{source:?}");
            let error = &parsed.errors[0];
            assert_eq!(error.message, message, "{source:?}");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
            assert_eq!(parsed.root.to_string(), source);
        }
        let missing = parse("(12.5").expect("small source");
        assert_eq!(
            missing
                .root
                .first_child()
                .expect("partial group")
                .to_string(),
            "(12.5"
        );
        let invalid = parse("(1 @)").expect("small source");
        let group = invalid.root.first_child().expect("group");
        assert_eq!(group.to_string(), "(1 @)");
        assert_eq!(invalid.root.children_with_tokens().count(), 1);
        let ending: Vec<_> = group
            .children_with_tokens()
            .skip(3)
            .map(|element| (element.kind(), element.to_string()))
            .collect();
        assert_eq!(
            ending,
            vec![
                (SyntaxKind::Error, "@".to_owned()),
                (SyntaxKind::RightParen, ")".to_owned())
            ]
        );
    }

    #[test]
    fn bounds_group_nesting_without_discarding_source() {
        let source = format!("{}1{}", "(".repeat(MAX_NESTING), ")".repeat(MAX_NESTING));
        let parsed = parse(&source).expect("small source");
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.root.to_string(), source);
        let too_deep = format!("({source})");
        let parsed = parse(&too_deep).expect("small source");
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].message,
            "expression nesting limit exceeded"
        );
        assert_eq!(usize::from(parsed.errors[0].range.start()), MAX_NESTING);
        assert_eq!(usize::from(parsed.errors[0].range.end()), MAX_NESTING + 1);
        assert_eq!(parsed.root.to_string(), too_deep);
    }

    #[test]
    fn unary_keeps_the_operator_and_intervening_trivia() {
        let source = "- // é\n12.5";
        let parsed = parse(source).expect("small source");
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.root.to_string(), source);
        let unary = parsed.root.first_child().expect("unary node");
        assert_eq!(unary.kind(), SyntaxKind::UnaryExpr);
        let children: Vec<_> = unary
            .children_with_tokens()
            .map(|child| {
                (
                    child.kind(),
                    usize::from(child.text_range().start()),
                    usize::from(child.text_range().end()),
                    child.to_string(),
                )
            })
            .collect();
        assert_eq!(
            children,
            vec![
                (SyntaxKind::Minus, 0, 1, "-".to_owned()),
                (SyntaxKind::Whitespace, 1, 2, " ".to_owned()),
                (SyntaxKind::LineComment, 2, 7, "// é".to_owned()),
                (SyntaxKind::Whitespace, 7, 8, "\n".to_owned()),
                (SyntaxKind::NumberExpr, 8, 12, "12.5".to_owned()),
            ]
        );
        let number = unary.first_child().expect("number node");
        assert_eq!(number.parent(), Some(unary));
        let spaced = parse(" -1 ").expect("small source");
        assert_eq!(spaced.root.children_with_tokens().count(), 3);
        assert_eq!(spaced.root.first_child().expect("unary").to_string(), "-1");
    }

    #[test]
    fn nests_unary_chains_and_groups() {
        for (source, kinds) in [
            (
                "--1",
                vec![
                    SyntaxKind::UnaryExpr,
                    SyntaxKind::UnaryExpr,
                    SyntaxKind::NumberExpr,
                ],
            ),
            (
                "-(001.50)",
                vec![
                    SyntaxKind::UnaryExpr,
                    SyntaxKind::GroupingExpr,
                    SyntaxKind::NumberExpr,
                ],
            ),
            (
                "(-1)",
                vec![
                    SyntaxKind::GroupingExpr,
                    SyntaxKind::UnaryExpr,
                    SyntaxKind::NumberExpr,
                ],
            ),
        ] {
            let parsed = parse(source).expect("small source");
            assert!(parsed.errors.is_empty(), "{source:?}");
            assert_eq!(parsed.root.to_string(), source);
            let mut node = parsed.root;
            for kind in kinds {
                node = node.first_child().expect("operand node");
                assert_eq!(node.kind(), kind, "{source:?}");
            }
        }
    }

    #[test]
    fn missing_unary_operands_keep_partial_nodes_and_suffixes() {
        for (source, message, start, end) in [
            ("-", "expected an expression", 1, 1),
            ("--", "expected an expression", 2, 2),
            ("(-)", "expected an expression", 2, 3),
            ("-é", "invalid token", 1, 3),
            ("- // é\n", "expected an expression", 8, 8),
            ("(-1", "expected ')'", 3, 3),
            ("-1)", "expected end of input", 2, 3),
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.root.to_string(), source);
            assert_eq!(parsed.errors.len(), 1, "{source:?}");
            let error = &parsed.errors[0];
            assert_eq!(error.message, message, "{source:?}");
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
        }
        let missing = parse("-").expect("small source");
        let unary = missing.root.first_child().expect("partial unary");
        assert_eq!(unary.kind(), SyntaxKind::UnaryExpr);
        let operand = unary.first_child().expect("missing operand slot");
        assert_eq!(operand.kind(), SyntaxKind::MissingExpr);
        assert_eq!(usize::from(operand.text_range().start()), 1);
        assert_eq!(usize::from(operand.text_range().end()), 1);
        assert_eq!(unary.to_string(), "-");
        let invalid = parse("-é").expect("small source");
        let unary = invalid.root.first_child().expect("unary");
        let skipped = unary.children_with_tokens().nth(1).expect("error token");
        assert_eq!(skipped.kind(), SyntaxKind::Error);
        assert_eq!(skipped.to_string(), "é");
    }

    #[test]
    fn shares_the_depth_budget_between_unary_and_groups() {
        for source in [
            format!("{}1", "-".repeat(MAX_NESTING)),
            format!("{}(1)", "-".repeat(MAX_NESTING - 1)),
            format!(
                "{}{}1{}",
                "(".repeat(MAX_NESTING / 2),
                "-".repeat(MAX_NESTING / 2),
                ")".repeat(MAX_NESTING / 2)
            ),
        ] {
            let parsed = parse(&source).expect("small source");
            assert!(parsed.errors.is_empty());
            assert_eq!(parsed.root.to_string(), source);
            let too_deep = format!("-{source}");
            let parsed = parse(&too_deep).expect("small source");
            assert_eq!(parsed.errors.len(), 1);
            assert_eq!(
                parsed.errors[0].message,
                "expression nesting limit exceeded"
            );
            assert_eq!(usize::from(parsed.errors[0].range.start()), MAX_NESTING);
            assert_eq!(usize::from(parsed.errors[0].range.end()), MAX_NESTING + 1);
            assert_eq!(parsed.root.to_string(), too_deep);
        }
    }

    fn shape(node: &SyntaxNode) -> String {
        match node.kind() {
            SyntaxKind::NumberExpr => node.to_string(),
            SyntaxKind::GroupingExpr => {
                format!("(group {})", shape(&node.first_child().expect("operand")))
            }
            SyntaxKind::UnaryExpr => {
                format!("(- {})", shape(&node.first_child().expect("operand")))
            }
            SyntaxKind::BinaryExpr => {
                let mut operands = node.children();
                let left = operands.next().expect("left operand");
                let right = operands.next().expect("right operand");
                let operator = node
                    .children_with_tokens()
                    .filter_map(|element| element.into_token())
                    .find(|token| {
                        matches!(
                            token.kind(),
                            SyntaxKind::Plus
                                | SyntaxKind::Minus
                                | SyntaxKind::Star
                                | SyntaxKind::Slash
                        )
                    })
                    .expect("operator");
                format!("({} {} {})", operator.text(), shape(&left), shape(&right))
            }
            kind => panic!("unexpected complete expression kind: {kind:?}"),
        }
    }

    #[test]
    fn builds_precedence_and_left_association() {
        for (source, expected) in [
            ("8 / 4 / 2", "(/ (/ 8 4) 2)"),
            ("8 / (4 / 2)", "(/ 8 (group (/ 4 2)))"),
            ("1 + 2 * 3", "(+ 1 (* 2 3))"),
            ("(1 + 2) * 3", "(* (group (+ 1 2)) 3)"),
            ("8 - 3 - 1", "(- (- 8 3) 1)"),
            ("1--2", "(- 1 (- 2))"),
            ("-(1 + 2) * 3", "(* (- (group (+ 1 2))) 3)"),
        ] {
            let parsed = parse(source).expect("small source");
            assert!(parsed.errors.is_empty(), "{source:?}");
            assert_eq!(parsed.root.to_string(), source);
            assert_eq!(
                shape(&parsed.root.first_child().expect("expression")),
                expected
            );
            assert_eq!(
                crate::parser::parse(source)
                    .expect("owned reference")
                    .to_prefix(),
                expected
            );
        }
    }

    #[test]
    fn wraps_only_expression_text_and_keeps_trivia_between_operands() {
        let source = " 001.0 // first\n + 02.00 * 3 ";
        let parsed = parse(source).expect("small source");
        assert!(parsed.errors.is_empty());
        assert_eq!(parsed.root.to_string(), source);
        let elements: Vec<_> = parsed.root.children_with_tokens().collect();
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0].kind(), SyntaxKind::Whitespace);
        assert_eq!(elements[2].kind(), SyntaxKind::Whitespace);
        let sum = parsed.root.first_child().expect("sum");
        assert_eq!(sum.to_string(), source.trim());
        let mut operands = sum.children();
        assert_eq!(operands.next().expect("left number").to_string(), "001.0");
        let product = operands.next().expect("right product");
        assert_eq!(product.kind(), SyntaxKind::BinaryExpr);
        assert_eq!(product.to_string(), "02.00 * 3");
        assert_eq!(product.parent(), Some(sum));
    }

    #[test]
    fn recovers_missing_binary_operands() {
        for (source, message, start, end) in [
            ("1 +", "expected an expression", 3, 3),
            ("8 **2", "expected an expression", 3, 4),
            ("1 + é", "invalid token", 4, 6),
            ("1 + (2 * )", "expected an expression", 9, 10),
            ("(1 + 2", "expected ')'", 6, 6),
            ("1 + 2 3", "expected end of input", 6, 7),
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.root.to_string(), source);
            assert_eq!(parsed.errors.len(), 1);
            let error = &parsed.errors[0];
            assert_eq!(error.message, message);
            assert_eq!(usize::from(error.range.start()), start);
            assert_eq!(usize::from(error.range.end()), end);
        }
        let failed = parse("1 +").expect("small source");
        let binary = failed.root.first_child().expect("partial binary");
        assert_eq!(binary.kind(), SyntaxKind::BinaryExpr);
        let kinds: Vec<_> = binary.children().map(|node| node.kind()).collect();
        assert_eq!(kinds, vec![SyntaxKind::NumberExpr, SyntaxKind::MissingExpr]);
    }

    #[test]
    fn bounds_both_binary_folds_without_discarding_source() {
        for separator in [" * 1", " + 1"] {
            let source = format!("1{}", separator.repeat(MAX_NESTING));
            let parsed = parse(&source).expect("small source");
            assert!(parsed.errors.is_empty());
            let too_deep = format!("{source}{separator}");
            let parsed = parse(&too_deep).expect("small source");
            assert_eq!(parsed.errors.len(), 1);
            assert_eq!(
                parsed.errors[0].message,
                "expression nesting limit exceeded"
            );
            assert_eq!(
                usize::from(parsed.errors[0].range.start()),
                2 + 4 * MAX_NESTING
            );
            assert_eq!(
                usize::from(parsed.errors[0].range.end()),
                3 + 4 * MAX_NESTING
            );
            assert_eq!(parsed.root.to_string(), too_deep);
        }
    }

    #[test]
    fn includes_both_operands_and_ancestor_wrappers_in_the_budget() {
        let factors = " * 1".repeat(MAX_NESTING - 1);
        let sums = " + 1".repeat(MAX_NESTING - 1);
        for source in [
            format!("1{factors} + 1"),
            format!("1 + 1{factors}"),
            format!("(1{sums})"),
            format!("-(1{})", " + 1".repeat(MAX_NESTING - 2)),
        ] {
            assert!(parse(&source).expect("small source").errors.is_empty());
        }
        let factors = " * 1".repeat(MAX_NESTING);
        let sums = " + 1".repeat(MAX_NESTING);
        for (source, operator) in [
            (format!("1{factors} + 1"), 2 + 4 * MAX_NESTING),
            (format!("1 + 1{factors}"), 2),
            (format!("(1{sums})"), 3 + 4 * (MAX_NESTING - 1)),
            (
                format!("-(1{})", " + 1".repeat(MAX_NESTING - 1)),
                4 + 4 * (MAX_NESTING - 2),
            ),
        ] {
            let parsed = parse(&source).expect("small source");
            assert_eq!(parsed.errors.len(), 1);
            assert_eq!(
                parsed.errors[0].message,
                "expression nesting limit exceeded"
            );
            assert_eq!(usize::from(parsed.errors[0].range.start()), operator);
            assert_eq!(usize::from(parsed.errors[0].range.end()), operator + 1);
            assert_eq!(parsed.root.to_string(), source);
        }
    }

    #[test]
    fn keeps_missing_operands_in_distinct_left_and_right_slots() {
        for (source, left, right, missing_at) in [
            ("1 +", SyntaxKind::NumberExpr, SyntaxKind::MissingExpr, 3),
            ("* 2", SyntaxKind::MissingExpr, SyntaxKind::NumberExpr, 0),
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.errors.len(), 1);
            assert_eq!(parsed.root.to_string(), source);
            let binary = parsed.root.first_child().expect("binary");
            assert_eq!(binary.kind(), SyntaxKind::BinaryExpr);
            let operands: Vec<_> = binary.children().collect();
            assert_eq!(operands.len(), 2);
            assert_eq!(operands[0].kind(), left);
            assert_eq!(operands[1].kind(), right);
            let missing = operands
                .iter()
                .find(|node| node.kind() == SyntaxKind::MissingExpr)
                .expect("missing slot");
            assert_eq!(usize::from(missing.text_range().start()), missing_at);
            assert_eq!(usize::from(missing.text_range().end()), missing_at);
            assert_eq!(missing.to_string(), "");
            assert_eq!(missing.children_with_tokens().count(), 0);
        }
    }

    #[test]
    fn recovers_group_closers_and_collects_multiple_errors() {
        let source = "(1 @) + é";
        let parsed = parse(source).expect("small source");
        assert_eq!(parsed.root.to_string(), source);
        let errors: Vec<_> = parsed
            .errors
            .iter()
            .map(|error| {
                (
                    error.message.as_str(),
                    usize::from(error.range.start()),
                    usize::from(error.range.end()),
                )
            })
            .collect();
        assert_eq!(
            errors,
            vec![("invalid token", 3, 4), ("invalid token", 8, 10)]
        );
        let binary = parsed.root.first_child().expect("binary");
        assert_eq!(binary.kind(), SyntaxKind::BinaryExpr);
        let mut operands = binary.children();
        assert_eq!(operands.next().expect("left group").to_string(), "(1 @)");
        assert_eq!(
            operands.next().expect("missing right").kind(),
            SyntaxKind::MissingExpr
        );
        let closed = parse("(-)").expect("small source");
        assert_eq!(closed.root.first_child().expect("group").to_string(), "(-)");
        assert_eq!(closed.root.children_with_tokens().count(), 1);
        let incomplete = parse("(1 + 2").expect("small source");
        let group = incomplete.root.first_child().expect("group");
        assert!(
            !group
                .children_with_tokens()
                .any(|element| element.kind() == SyntaxKind::RightParen)
        );
        assert_eq!(incomplete.root.to_string(), "(1 + 2");
    }

    #[test]
    fn retains_unowned_closers_and_other_root_suffix_tokens() {
        let source = "1 ) é";
        let parsed = parse(source).expect("small source");
        assert_eq!(parsed.root.to_string(), source);
        let errors: Vec<_> = parsed
            .errors
            .iter()
            .map(|error| {
                (
                    error.message.as_str(),
                    usize::from(error.range.start()),
                    usize::from(error.range.end()),
                )
            })
            .collect();
        assert_eq!(
            errors,
            vec![("expected end of input", 2, 3), ("invalid token", 4, 6)]
        );
    }

    #[test]
    fn terminates_on_malformed_shapes_and_stops_at_resource_limits() {
        for source in [
            "", ")", "((( ", "1 + * 2", "@-é", "1 ) @", "((1 + ))", "++++", "****",
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.root.to_string(), source);
            assert_eq!(usize::from(parsed.root.text_range().end()), source.len());
            assert!(!parsed.errors.is_empty());
        }
        for source in [
            format!(
                "{}1{}",
                "(".repeat(MAX_NESTING + 1),
                ")".repeat(MAX_NESTING + 1)
            ),
            "*".repeat(MAX_NESTING + 6),
        ] {
            let parsed = parse(&source).expect("small source");
            assert_eq!(parsed.root.to_string(), source);
            assert_eq!(
                parsed.errors.last().expect("limit error").message,
                "expression nesting limit exceeded"
            );
            assert!(
                !parsed
                    .errors
                    .iter()
                    .any(|error| error.message == "expected end of input")
            );
        }
    }
}
