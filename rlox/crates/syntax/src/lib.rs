use rlox_text::{TextRange, TextSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    LeftParen,
    RightParen,
    Bang,
    Equal,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub range: TextRange,
}

pub fn scan(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (start, ch) in source.char_indices() {
        let kind = match ch {
            '(' => SyntaxKind::LeftParen,
            ')' => SyntaxKind::RightParen,
            '!' => SyntaxKind::Bang,
            '=' => SyntaxKind::Equal,
            _ => SyntaxKind::Error,
        };
        let end = start + ch.len_utf8();
        let range = TextRange::new(TextSize(start), TextSize(end))
            .expect("character end follows its start");

        tokens.push(Token { kind, range });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::{SyntaxKind, Token, scan};
    use rlox_text::{TextRange, TextSize};

    #[test]
    fn scans_single_character_tokens() {
        let tokens = scan("()!=");

        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: SyntaxKind::LeftParen,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::RightParen,
                    range: TextRange::new(TextSize(1), TextSize(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Bang,
                    range: TextRange::new(TextSize(2), TextSize(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Equal,
                    range: TextRange::new(TextSize(3), TextSize(4)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn empty_source_has_no_tokens() {
        assert!(scan("").is_empty());
    }

    #[test]
    fn consumes_a_whole_unsupported_character() {
        let source = "é!";
        let tokens = scan(source);

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, SyntaxKind::Error);
        assert_eq!(tokens[0].range.slice(source), Some("é"));
        assert_eq!(tokens[1].kind, SyntaxKind::Bang);
        assert_eq!(tokens[1].range.slice(source), Some("!"));
    }
}
