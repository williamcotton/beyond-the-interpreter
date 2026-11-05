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
    let mut errors = Vec::new();
    match parser.expression(0) {
        Err(error) => errors.push(error),
        Ok(()) => {
            parser.eat_trivia();
            if parser.peek().is_some() {
                errors.push(parser.error("expected end of input"));
            }
        }
    }

    while parser.current < parser.tokens.len() {
        parser.bump();
    }
    parser.builder.finish_node();
    let root = SyntaxNode::new_root(parser.builder.finish());
    Ok(Parse { root, errors })
}

struct Parser<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    current: usize,
    builder: GreenNodeBuilder<'static>,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Result<Self, SourceTooLarge> {
        Ok(Self {
            source,
            tokens: scan(source)?,
            current: 0,
            builder: GreenNodeBuilder::new(),
        })
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).copied()
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

    fn expression(&mut self, nesting: usize) -> Result<(), ParseError> {
        self.unary(nesting)
    }

    fn unary(&mut self, nesting: usize) -> Result<(), ParseError> {
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
                result
            }
            _ => self.primary(nesting),
        }
    }

    fn primary(&mut self, nesting: usize) -> Result<(), ParseError> {
        match self.peek() {
            Some(token) if token.kind == SyntaxKind::Number => {
                self.builder.start_node(SyntaxKind::NumberExpr.into_raw());
                self.bump();
                self.builder.finish_node();
                Ok(())
            }
            Some(token) if token.kind == SyntaxKind::LeftParen => self.group(nesting),
            _ => Err(self.error("expected an expression")),
        }
    }

    fn group(&mut self, nesting: usize) -> Result<(), ParseError> {
        if nesting >= MAX_NESTING {
            return Err(self.error("expression nesting limit exceeded"));
        }
        self.builder.start_node(SyntaxKind::GroupingExpr.into_raw());
        self.bump();
        let result = match self.expression(nesting + 1) {
            Err(error) => Err(error),
            Ok(()) => {
                self.eat_trivia();
                match self.peek() {
                    Some(token) if token.kind == SyntaxKind::RightParen => {
                        self.bump();
                        Ok(())
                    }
                    _ => Err(self.error("expected ')'")),
                }
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
    fn preserves_source_for_success_and_first_failure() {
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
            assert!(parsed.errors.len() <= 1);
        }
        for source in ["001.50", "((12.5))", " // é\n(12.5) // end"] {
            assert!(parse(source).expect("small source").errors.is_empty());
        }
    }

    #[test]
    fn reports_located_errors_and_retains_the_suffix() {
        for (source, message, start, end) in [
            ("", "expected an expression", 0, 0),
            (" // é\n", "expected an expression", 7, 7),
            ("(12.5", "expected ')'", 5, 5),
            ("()", "expected an expression", 1, 2),
            ("(1 @)", "invalid token", 3, 4),
            ("é", "invalid token", 0, 2),
            ("1 2", "expected end of input", 2, 3),
            ("1 + 2", "expected end of input", 2, 3),
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
        assert_eq!(
            invalid
                .root
                .first_child()
                .expect("partial group")
                .to_string(),
            "(1 "
        );
        let suffix: Vec<_> = invalid
            .root
            .children_with_tokens()
            .skip(1)
            .map(|element| (element.kind(), element.to_string()))
            .collect();
        assert_eq!(
            suffix,
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
        assert_eq!(unary.children().count(), 0);
        assert_eq!(unary.to_string(), "-");
        let invalid = parse("-é").expect("small source");
        let suffix = invalid
            .root
            .children_with_tokens()
            .nth(1)
            .expect("error token");
        assert_eq!(suffix.kind(), SyntaxKind::Error);
        assert_eq!(suffix.to_string(), "é");
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
}
