use crate::SyntaxKind;
use crate::tree::{SyntaxNode, SyntaxToken};

#[derive(Debug, Clone)]
pub struct NumberExpr {
    syntax: SyntaxNode,
}
#[derive(Debug, Clone)]
pub struct GroupingExpr {
    syntax: SyntaxNode,
}
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    syntax: SyntaxNode,
}
#[derive(Debug, Clone)]
pub struct BinaryExpr {
    syntax: SyntaxNode,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(NumberExpr),
    Grouping(GroupingExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl Expr {
    pub fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::NumberExpr => Some(Self::Number(NumberExpr { syntax })),
            SyntaxKind::GroupingExpr => Some(Self::Grouping(GroupingExpr { syntax })),
            SyntaxKind::UnaryExpr => Some(Self::Unary(UnaryExpr { syntax })),
            SyntaxKind::BinaryExpr => Some(Self::Binary(BinaryExpr { syntax })),
            _ => None,
        }
    }

    pub fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Number(node) => node.syntax(),
            Self::Grouping(node) => node.syntax(),
            Self::Unary(node) => node.syntax(),
            Self::Binary(node) => node.syntax(),
        }
    }

    pub fn to_prefix(&self) -> Option<String> {
        match self {
            Self::Number(node) => {
                let token = node.token()?;
                let value = token.text().parse::<f64>().ok()?;
                Some(value.to_string())
            }
            Self::Grouping(node) => {
                node.l_paren_token()?;
                node.r_paren_token()?;
                Some(format!("(group {})", node.inner()?.to_prefix()?))
            }
            Self::Unary(node) => Some(format!(
                "({} {})",
                node.operator_token()?.text(),
                node.operand()?.to_prefix()?
            )),
            Self::Binary(node) => Some(format!(
                "({} {} {})",
                node.operator_token()?.text(),
                node.left()?.to_prefix()?,
                node.right()?.to_prefix()?
            )),
        }
    }
}

impl NumberExpr {
    pub fn cast(syntax: SyntaxNode) -> Option<Self> {
        if syntax.kind() != SyntaxKind::NumberExpr {
            return None;
        }
        Some(Self { syntax })
    }

    pub fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }

    pub fn token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::Number)
    }
}

impl GroupingExpr {
    pub fn cast(syntax: SyntaxNode) -> Option<Self> {
        if syntax.kind() != SyntaxKind::GroupingExpr {
            return None;
        }
        Some(Self { syntax })
    }

    pub fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }

    pub fn inner(&self) -> Option<Expr> {
        self.syntax.first_child().and_then(Expr::cast)
    }

    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::LeftParen)
    }

    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::RightParen)
    }
}

impl UnaryExpr {
    pub fn cast(syntax: SyntaxNode) -> Option<Self> {
        if syntax.kind() != SyntaxKind::UnaryExpr {
            return None;
        }
        Some(Self { syntax })
    }

    pub fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }

    pub fn operator_token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::Minus)
    }

    pub fn operand(&self) -> Option<Expr> {
        self.syntax.first_child().and_then(Expr::cast)
    }
}

impl BinaryExpr {
    pub fn cast(syntax: SyntaxNode) -> Option<Self> {
        if syntax.kind() != SyntaxKind::BinaryExpr {
            return None;
        }
        Some(Self { syntax })
    }

    pub fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }

    pub fn left(&self) -> Option<Expr> {
        self.syntax.first_child().and_then(Expr::cast)
    }

    pub fn right(&self) -> Option<Expr> {
        self.syntax.children().nth(1).and_then(Expr::cast)
    }

    pub fn operator_token(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Star | SyntaxKind::Slash
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryExpr, Expr, GroupingExpr, NumberExpr, UnaryExpr};
    use crate::{SyntaxKind, cst::parse};

    #[test]
    fn casts_by_receiver_kind_without_copying_source() {
        let parsed = parse("001.50").expect("small source");
        assert!(parsed.errors.is_empty());
        let raw = parsed.root.first_child().expect("number");
        assert!(BinaryExpr::cast(raw.clone()).is_none());
        assert!(GroupingExpr::cast(raw.clone()).is_none());
        assert!(UnaryExpr::cast(raw.clone()).is_none());
        assert!(Expr::cast(parsed.root.clone()).is_none());
        let number = NumberExpr::cast(raw.clone()).expect("number view");
        assert_eq!(number.syntax(), &raw);
        assert_eq!(number.token().expect("number token").text(), "001.50");
        let expr = Expr::cast(raw).expect("expression view");
        assert_eq!(expr.syntax().to_string(), "001.50");
        assert_eq!(expr.to_prefix().as_deref(), Some("1.5"));
    }

    #[test]
    fn missing_slots_do_not_shift_binary_operands() {
        for (source, has_left, has_right) in [("1 +", true, false), ("* 2", false, true)] {
            let parsed = parse(source).expect("small source");
            assert!(!parsed.errors.is_empty());
            let raw = parsed.root.first_child().expect("binary");
            let binary = BinaryExpr::cast(raw.clone()).expect("binary view");
            assert_eq!(binary.left().is_some(), has_left);
            assert_eq!(binary.right().is_some(), has_right);
            assert_eq!(binary.syntax().to_string(), source);
            if let Some(right) = binary.right() {
                assert_eq!(right.syntax().to_string(), "2");
                assert_eq!(usize::from(right.syntax().text_range().start()), 2);
            }
            assert!(Expr::cast(raw).expect("expression").to_prefix().is_none());
        }
    }

    #[test]
    fn partial_group_and_unary_views_keep_actual_tokens() {
        let parsed = parse("-").expect("small source");
        let unary = UnaryExpr::cast(parsed.root.first_child().expect("unary")).expect("unary view");
        assert_eq!(unary.operator_token().expect("operator").text(), "-");
        assert!(unary.operand().is_none());
        let parsed = parse("(1").expect("small source");
        let group =
            GroupingExpr::cast(parsed.root.first_child().expect("group")).expect("group view");
        assert!(group.l_paren_token().is_some());
        assert!(group.r_paren_token().is_none());
        assert_eq!(group.inner().expect("number").syntax().to_string(), "1");
        assert!(
            Expr::cast(group.syntax().clone())
                .expect("expression")
                .to_prefix()
                .is_none()
        );
    }

    #[test]
    fn accessors_follow_structure_and_preserve_positions() {
        let parsed = parse("1 + 2 * 3").expect("small source");
        assert!(parsed.errors.is_empty());
        let sum = BinaryExpr::cast(parsed.root.first_child().expect("sum")).expect("binary view");
        assert_eq!(
            sum.operator_token().expect("operator").kind(),
            SyntaxKind::Plus
        );
        let right = sum.right().expect("product");
        let Expr::Binary(product) = right else {
            panic!("right factor must be binary")
        };
        assert_eq!(product.operator_token().expect("operator").text(), "*");
        assert_eq!(usize::from(product.syntax().text_range().start()), 4);
        assert_eq!(usize::from(product.syntax().text_range().end()), 9);
        assert_eq!(
            product.left().expect("left number").to_prefix().as_deref(),
            Some("2")
        );
        assert_eq!(
            product
                .right()
                .expect("right number")
                .to_prefix()
                .as_deref(),
            Some("3")
        );
    }

    #[test]
    fn preserves_the_existing_inspection_corpus() {
        for (source, expected) in [
            (" 123 // note\n", Some("123")),
            ("123 456", None),
            ("", None),
            ("é", None),
            ("(123)", Some("(group 123)")),
            ("((12.5))", Some("(group (group 12.5))")),
            ("( // é\n123 )", Some("(group 123)")),
            ("()", None),
            ("(123", None),
            ("(123 456)", None),
            ("123)", None),
            ("-123", Some("(- 123)")),
            ("--123", Some("(- (- 123))")),
            ("-(123)", Some("(- (group 123))")),
            ("(-123)", Some("(group (- 123))")),
            ("-", None),
            ("(-)", None),
            ("-é", None),
            ("8 / 4 / 2", Some("(/ (/ 8 4) 2)")),
            ("8 * 4 / 2", Some("(/ (* 8 4) 2)")),
            ("8 / (4 / 2)", Some("(/ 8 (group (/ 4 2)))")),
            ("-8 * 4 / 2", Some("(/ (* (- 8) 4) 2)")),
            ("8 *", None),
            ("8 **2", None),
            ("8 / é", None),
            ("1 + 2 * 3", Some("(+ 1 (* 2 3))")),
            ("(1 + 2) * 3", Some("(* (group (+ 1 2)) 3)")),
            ("8 - 3 - 1", Some("(- (- 8 3) 1)")),
            ("1--2", Some("(- 1 (- 2))")),
            ("-(1 + 2) * 3", Some("(* (- (group (+ 1 2))) 3)")),
            ("1 +", None),
            ("1 + +2", None),
            ("1 + é", None),
            ("(1 + 2", None),
            (" 001.0 // first\n + 02.00 * 3", Some("(+ 1 (* 2 3))")),
        ] {
            let parsed = parse(source).expect("small source");
            assert_eq!(parsed.root.to_string(), source);
            if let Some(expected) = expected {
                assert!(parsed.errors.is_empty(), "{source:?}");
                let view =
                    Expr::cast(parsed.root.first_child().expect("expression")).expect("typed view");
                assert_eq!(view.to_prefix().as_deref(), Some(expected), "{source:?}");
            } else {
                assert!(!parsed.errors.is_empty(), "{source:?}");
            }
        }
    }
}
