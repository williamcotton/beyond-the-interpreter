use logos::Logos;

use rlox_text::{TextRange, TextSize};

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxKind {
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("fun")]
    Fun,
    #[token("if")]
    If,
    #[token("nil")]
    Nil,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("return")]
    Return,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("true")]
    True,
    #[token("var")]
    Var,
    #[token("while")]
    While,
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,
    #[regex(r"[ \t\r\n]+")]
    Whitespace,
    #[regex(r"//[^\n]*", allow_greedy = true)]
    LineComment,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub range: TextRange,
}

pub fn scan(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (result, span) in SyntaxKind::lexer(source).spanned() {
        let kind = match result {
            Ok(kind) => kind,
            Err(()) => SyntaxKind::Error,
        };
        let range = TextRange::new(TextSize(span.start), TextSize(span.end))
            .expect("Logos spans are ordered");

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
        for source in ["_", "A9", "printer"] {
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
            "/",
            "/name",
            "//",
            "//\n",
            "//é\r\nname",
            "///",
            "a//x\nb",
            "{},.-+;*",
            "<=>=<>",
            "<==>==",
            "a<=b!=c>=d",
            "(a+b)*c;",
            "print printer",
            "for forest",
            "class classify",
            "if(if_1)",
            "return // print\nprinter",
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

    #[test]
    fn recognizes_single_slashes() {
        assert_eq!(
            scan("/"),
            vec![Token {
                kind: SyntaxKind::Slash,
                range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
            }]
        );
        assert_eq!(
            scan("/name"),
            vec![
                Token {
                    kind: SyntaxKind::Slash,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(1), TextSize(5)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn line_comments_leave_the_line_feed_unread() {
        assert_eq!(
            scan("//é\r\nname"),
            vec![
                Token {
                    kind: SyntaxKind::LineComment,
                    range: TextRange::new(TextSize(0), TextSize(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: TextRange::new(TextSize(5), TextSize(6)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(6), TextSize(10)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn line_comments_can_end_at_eof() {
        for source in ["//", "//é", "///"] {
            assert_eq!(
                scan(source),
                vec![Token {
                    kind: SyntaxKind::LineComment,
                    range: TextRange::new(TextSize(0), TextSize(source.len())).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn unknown_characters_remain_separate_tokens() {
        assert_eq!(
            scan("@é💥"),
            vec![
                Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(0), TextSize(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(1), TextSize(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: TextRange::new(TextSize(3), TextSize(7)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn recognizes_remaining_single_character_punctuation() {
        for (source, kind) in [
            ("{", SyntaxKind::LeftBrace),
            ("}", SyntaxKind::RightBrace),
            (",", SyntaxKind::Comma),
            (".", SyntaxKind::Dot),
            ("-", SyntaxKind::Minus),
            ("+", SyntaxKind::Plus),
            (";", SyntaxKind::Semicolon),
            ("*", SyntaxKind::Star),
            ("<", SyntaxKind::Less),
            (">", SyntaxKind::Greater),
        ] {
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
    fn recognizes_adjacent_comparison_operators() {
        assert_eq!(
            scan("<=>=<>"),
            vec![
                Token {
                    kind: SyntaxKind::LessEqual,
                    range: TextRange::new(TextSize(0), TextSize(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::GreaterEqual,
                    range: TextRange::new(TextSize(2), TextSize(4)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Less,
                    range: TextRange::new(TextSize(4), TextSize(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Greater,
                    range: TextRange::new(TextSize(5), TextSize(6)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn comparison_pairs_leave_extra_equals_tokens() {
        assert_eq!(
            scan("<==>=="),
            vec![
                Token {
                    kind: SyntaxKind::LessEqual,
                    range: TextRange::new(TextSize(0), TextSize(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Equal,
                    range: TextRange::new(TextSize(2), TextSize(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::GreaterEqual,
                    range: TextRange::new(TextSize(3), TextSize(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Equal,
                    range: TextRange::new(TextSize(5), TextSize(6)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn recognizes_reserved_words() {
        for (source, kind) in [
            ("and", SyntaxKind::And),
            ("class", SyntaxKind::Class),
            ("else", SyntaxKind::Else),
            ("false", SyntaxKind::False),
            ("for", SyntaxKind::For),
            ("fun", SyntaxKind::Fun),
            ("if", SyntaxKind::If),
            ("nil", SyntaxKind::Nil),
            ("or", SyntaxKind::Or),
            ("print", SyntaxKind::Print),
            ("return", SyntaxKind::Return),
            ("super", SyntaxKind::Super),
            ("this", SyntaxKind::This),
            ("true", SyntaxKind::True),
            ("var", SyntaxKind::Var),
            ("while", SyntaxKind::While),
        ] {
            assert_eq!(
                scan(source),
                vec![Token {
                    kind,
                    range: TextRange::new(TextSize(0), TextSize(source.len())).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn keywords_do_not_split_longer_identifiers() {
        for source in [
            "forest",
            "printer",
            "classify",
            "true_value",
            "_if",
            "Print",
        ] {
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
    fn token_spelling_comes_from_the_source() {
        let source = "print printer";
        let tokens = scan(source);

        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: SyntaxKind::Print,
                    range: TextRange::new(TextSize(0), TextSize(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: TextRange::new(TextSize(5), TextSize(6)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: TextRange::new(TextSize(6), TextSize(13)).unwrap(),
                },
            ]
        );
        assert_eq!(tokens[0].range.slice(source), Some("print"));
        assert_eq!(tokens[1].range.slice(source), Some(" "));
        assert_eq!(tokens[2].range.slice(source), Some("printer"));
    }
}
