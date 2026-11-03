use rlox_text::{TextRange, TextSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    LeftParen,
    RightParen,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Identifier,
    Whitespace,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub range: TextRange,
}

pub fn scan(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        let mut end = start + ch.len_utf8();
        let kind = match ch {
            '(' => SyntaxKind::LeftParen,
            ')' => SyntaxKind::RightParen,
            '!' => {
                if chars.next_if(|&(_, next)| next == '=').is_some() {
                    end += 1;
                    SyntaxKind::BangEqual
                } else {
                    SyntaxKind::Bang
                }
            }
            '=' => {
                if chars.next_if(|&(_, next)| next == '=').is_some() {
                    end += 1;
                    SyntaxKind::EqualEqual
                } else {
                    SyntaxKind::Equal
                }
            }
            ' ' | '\t' | '\r' | '\n' => {
                while let Some((offset, next)) =
                    chars.next_if(|&(_, next)| matches!(next, ' ' | '\t' | '\r' | '\n'))
                {
                    end = offset + next.len_utf8();
                }
                SyntaxKind::Whitespace
            }
            ch if ch.is_ascii_alphabetic() || ch == '_' => {
                while let Some((offset, next)) =
                    chars.next_if(|&(_, next)| next.is_ascii_alphanumeric() || next == '_')
                {
                    end = offset + next.len_utf8();
                }
                SyntaxKind::Identifier
            }
            _ => SyntaxKind::Error,
        };
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
        let tokens = scan("()=!");

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
                    kind: SyntaxKind::Equal,
                    range: TextRange::new(TextSize(2), TextSize(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Bang,
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

    #[test]
    fn scans_adjacent_paired_operators() {
        assert_eq!(
            scan("!==="),
            vec![
                Token {
                    kind: SyntaxKind::BangEqual,
                    range: TextRange::new(TextSize(0), TextSize(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::EqualEqual,
                    range: TextRange::new(TextSize(2), TextSize(4)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn failed_lookahead_preserves_the_next_character() {
        assert_eq!(
            scan("!é("),
            vec![
                Token {
                    kind: SyntaxKind::Bang,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(1), TextSize(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::LeftParen,
                    range: TextRange::new(TextSize(3), TextSize(4)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn single_operator_can_end_the_source() {
        for (source, kind) in [("!", SyntaxKind::Bang), ("=", SyntaxKind::Equal)] {
            assert_eq!(
                scan(source),
                vec![Token {
                    kind,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn identifiers_stop_before_operators() {
        assert_eq!(
            scan("a_2!=other"),
            vec![
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(0), TextSize(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::BangEqual,
                    range: TextRange::new(TextSize(3), TextSize(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(5), TextSize(10)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn identifiers_can_end_the_source() {
        for source in ["_", "A9", "print"] {
            assert_eq!(
                scan(source),
                vec![Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(0), TextSize(source.len())).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn identifier_rules_preserve_unsupported_characters() {
        assert_eq!(
            scan("2aé_b"),
            vec![
                Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(1), TextSize(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(2), TextSize(4)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(4), TextSize(6)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn keeps_whitespace_between_tokens() {
        assert_eq!(
            scan("a \t\r\n!= b"),
            vec![
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: TextRange::new(TextSize(1), TextSize(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::BangEqual,
                    range: TextRange::new(TextSize(5), TextSize(7)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: TextRange::new(TextSize(7), TextSize(8)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(8), TextSize(9)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn whitespace_uses_the_lox_character_set() {
        for source in ["\u{000b}", "\u{000c}", "\u{00a0}"] {
            assert_eq!(
                scan(source),
                vec![Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(0), TextSize(source.len())).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn token_ranges_cover_the_original_source() {
        for source in [
            "",
            " \t\r\n",
            "a \t\r\n!= b",
            "a_2!=other",
            "2aé_b",
            "a\u{00a0}b",
        ] {
            let mut offset = 0;
            let mut reconstructed = String::new();

            for token in scan(source) {
                let text = token.range.slice(source).expect("valid UTF-8 source slice");
                assert!(!text.is_empty());
                let end = offset + text.len();
                let expected = TextRange::new(TextSize(offset), TextSize(end)).unwrap();
                assert_eq!(token.range, expected);

                reconstructed.push_str(text);
                offset = end;
            }

            assert_eq!(offset, source.len());
            assert_eq!(reconstructed, source);
        }
    }
}
