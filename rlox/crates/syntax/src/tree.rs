use rlox_text::{SourceTooLarge, slice};
use rowan::GreenNodeBuilder;

use crate::{SyntaxKind, scan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoxLanguage {}

impl rowan::Language for LoxLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from_raw(raw).expect("tree contains an unknown Lox kind")
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        kind.into_raw()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<LoxLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<LoxLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<LoxLanguage>;

pub fn token_tree(source: &str) -> Result<SyntaxNode, SourceTooLarge> {
    let tokens = scan(source)?;
    let mut builder = GreenNodeBuilder::new();
    builder.start_node(SyntaxKind::Root.into_raw());

    for token in tokens {
        let spelling = slice(source, token.range).expect("scanner ranges are valid");
        if token.kind == SyntaxKind::Number {
            builder.start_node(SyntaxKind::NumberExpr.into_raw());
        }
        builder.token(token.kind.into_raw(), spelling);
        if token.kind == SyntaxKind::Number {
            builder.finish_node();
        }
    }

    builder.finish_node();
    let green = builder.finish();
    Ok(SyntaxNode::new_root(green))
}

#[cfg(test)]
mod tests {
    use super::token_tree;
    use crate::SyntaxKind;

    #[test]
    fn preserves_all_source_even_without_an_expression() {
        for source in ["", "001.50", " // é\n12.5 // end", "@é!", "1 + 2"] {
            let root = token_tree(source).expect("small source");
            assert_eq!(root.kind(), SyntaxKind::Root);
            assert_eq!(root.to_string(), source);
            assert_eq!(usize::from(root.text_range().start()), 0);
            assert_eq!(usize::from(root.text_range().end()), source.len());
            assert!(root.parent().is_none());
        }
    }

    #[test]
    fn positions_number_and_trivia_without_losing_their_relationships() {
        let root = token_tree(" // é\n12.5 // end").expect("small source");
        let number = root.first_child().expect("number node");
        assert_eq!(number.kind(), SyntaxKind::NumberExpr);
        assert_eq!(number.parent(), Some(root.clone()));
        let token = number
            .children_with_tokens()
            .next()
            .expect("number token")
            .into_token()
            .expect("a token child");
        assert_eq!(token.kind(), SyntaxKind::Number);
        assert_eq!(token.text(), "12.5");
        assert_eq!(token.parent(), Some(number.clone()));
        assert_eq!(token.text_range(), number.text_range());
        let children: Vec<_> = root
            .children_with_tokens()
            .map(|element| {
                (
                    element.kind(),
                    usize::from(element.text_range().start()),
                    usize::from(element.text_range().end()),
                )
            })
            .collect();
        assert_eq!(
            children,
            vec![
                (SyntaxKind::Whitespace, 0, 1),
                (SyntaxKind::LineComment, 1, 6),
                (SyntaxKind::Whitespace, 6, 7),
                (SyntaxKind::NumberExpr, 7, 11),
                (SyntaxKind::Whitespace, 11, 12),
                (SyntaxKind::LineComment, 12, 18),
            ]
        );
    }

    #[test]
    fn keeps_invalid_token_spelling() {
        let root = token_tree("@é!").expect("representable source");
        let tokens: Vec<_> = root
            .children_with_tokens()
            .map(|element| {
                let token = element.into_token().expect("no number nodes");
                (
                    token.kind(),
                    token.text().to_owned(),
                    usize::from(token.text_range().start()),
                    usize::from(token.text_range().end()),
                )
            })
            .collect();
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::Error, "@".to_owned(), 0, 1),
                (SyntaxKind::Error, "é".to_owned(), 1, 3),
                (SyntaxKind::Bang, "!".to_owned(), 3, 4),
            ]
        );
    }
}
