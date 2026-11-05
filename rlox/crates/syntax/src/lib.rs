pub mod cst;
pub mod expr;
pub mod parser;
pub mod tree;

use logos::Logos;

use rlox_text::{SourceTooLarge, TextRange, checked_range, checked_size};

macro_rules! syntax_kinds {
    ($( $(#[$attribute:meta])* $kind:ident, )*) => {
        #[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum SyntaxKind {
            $( $(#[$attribute])* $kind, )*
        }

        impl SyntaxKind {
            pub fn into_raw(self) -> rowan::SyntaxKind {
                rowan::SyntaxKind(self as u16)
            }

            pub fn from_raw(raw: rowan::SyntaxKind) -> Option<Self> {
                match raw.0 {
                    $(value if value == Self::$kind as u16 => Some(Self::$kind),)*
                    _ => None,
                }
            }

            #[cfg(test)]
            const ALL: &'static [Self] = &[$(Self::$kind,)*];
        }
    };
}

syntax_kinds! {
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
    #[regex(r#""[^"]*""#)]
    String,
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier,
    #[regex(r"[ \t\r\n]+")]
    Whitespace,
    #[regex(r"//[^\n]*", allow_greedy = true)]
    LineComment,
    Error,
    Root,
    NumberExpr,
    GroupingExpr,
    UnaryExpr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub range: TextRange,
}

pub fn scan(source: &str) -> Result<Vec<Token>, SourceTooLarge> {
    checked_size(source.len())?;
    let mut tokens = Vec::new();

    for (result, span) in SyntaxKind::lexer(source).spanned() {
        let kind = match result {
            Ok(kind) => kind,
            Err(()) => SyntaxKind::Error,
        };
        let range = checked_range(
            checked_size(span.start).expect("validated source length"),
            checked_size(span.end).expect("validated source length"),
        )
        .expect("Logos spans are ordered");

        tokens.push(Token { kind, range });
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{SyntaxKind, Token, scan};
    use rlox_text::{TextSize, checked_range, checked_size, slice};

    #[test]
    fn scans_single_character_tokens() {
        let tokens = scan("()=!").expect("small test source");

        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: SyntaxKind::LeftParen,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::RightParen,
                    range: checked_range(TextSize::new(1), TextSize::new(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Equal,
                    range: checked_range(TextSize::new(2), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Bang,
                    range: checked_range(TextSize::new(3), TextSize::new(4)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn empty_source_has_no_tokens() {
        assert!(scan("").expect("small test source").is_empty());
    }

    #[test]
    fn consumes_a_whole_unsupported_character() {
        let source = "é!";
        let tokens = scan(source).expect("small test source");

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, SyntaxKind::Error);
        assert_eq!(slice(source, tokens[0].range), Some("é"));
        assert_eq!(tokens[1].kind, SyntaxKind::Bang);
        assert_eq!(slice(source, tokens[1].range), Some("!"));
    }

    #[test]
    fn scans_adjacent_paired_operators() {
        assert_eq!(
            scan("!===").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::BangEqual,
                    range: checked_range(TextSize::new(0), TextSize::new(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::EqualEqual,
                    range: checked_range(TextSize::new(2), TextSize::new(4)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn failed_lookahead_preserves_the_next_character() {
        assert_eq!(
            scan("!é(").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::Bang,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(TextSize::new(1), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::LeftParen,
                    range: checked_range(TextSize::new(3), TextSize::new(4)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn single_operator_can_end_the_source() {
        for (source, kind) in [("!", SyntaxKind::Bang), ("=", SyntaxKind::Equal)] {
            assert_eq!(
                scan(source).expect("small test source"),
                vec![Token {
                    kind,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn identifiers_stop_before_operators() {
        assert_eq!(
            scan("a_2!=other").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(0), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::BangEqual,
                    range: checked_range(TextSize::new(3), TextSize::new(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(5), TextSize::new(10)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn identifiers_can_end_the_source() {
        for source in ["_", "A9", "printer"] {
            assert_eq!(
                scan(source).expect("small test source"),
                vec![Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
                }]
            );
        }
    }

    #[test]
    fn number_and_name_boundaries_preserve_unknown_characters() {
        assert_eq!(
            scan("2aé_b").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::Number,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(1), TextSize::new(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(TextSize::new(2), TextSize::new(4)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(4), TextSize::new(6)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn keeps_whitespace_between_tokens() {
        assert_eq!(
            scan("a \t\r\n!= b").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: checked_range(TextSize::new(1), TextSize::new(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::BangEqual,
                    range: checked_range(TextSize::new(5), TextSize::new(7)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: checked_range(TextSize::new(7), TextSize::new(8)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(8), TextSize::new(9)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn whitespace_uses_the_lox_character_set() {
        for source in ["\u{000b}", "\u{000c}", "\u{00a0}"] {
            assert_eq!(
                scan(source).expect("small test source"),
                vec![Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
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
            "0 001 123.456",
            "123. .5 -12 +3.5 1.2.3",
            "123abc a2",
            "print 0.5;//é\n123.",
            r#"print "é";"#,
            r#""a""b";"#,
            r#""\"name"#,
            "\"first\nsecond\" // done\n0.5",
            "\"",
            "\"é",
            "\"first\n// still inside",
            r#""ok""é"#,
        ] {
            let mut offset = 0;
            let mut reconstructed = String::new();

            for token in scan(source).expect("small test source") {
                let text = slice(source, token.range).expect("valid UTF-8 source slice");
                assert!(!text.is_empty());
                let end = offset + text.len();
                let expected = checked_range(
                    checked_size(offset).expect("representable offset"),
                    checked_size(end).expect("representable offset"),
                )
                .unwrap();
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
            scan("/").expect("small test source"),
            vec![Token {
                kind: SyntaxKind::Slash,
                range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
            }]
        );
        assert_eq!(
            scan("/name").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::Slash,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(1), TextSize::new(5)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn line_comments_leave_the_line_feed_unread() {
        assert_eq!(
            scan("//é\r\nname").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::LineComment,
                    range: checked_range(TextSize::new(0), TextSize::new(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: checked_range(TextSize::new(5), TextSize::new(6)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(6), TextSize::new(10)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn line_comments_can_end_at_eof() {
        for source in ["//", "//é", "///"] {
            assert_eq!(
                scan(source).expect("small test source"),
                vec![Token {
                    kind: SyntaxKind::LineComment,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
                }]
            );
        }
    }

    #[test]
    fn unknown_characters_remain_separate_tokens() {
        assert_eq!(
            scan("@é💥").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(TextSize::new(1), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(TextSize::new(3), TextSize::new(7)).unwrap(),
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
                scan(source).expect("small test source"),
                vec![Token {
                    kind,
                    range: checked_range(TextSize::new(0), TextSize::new(1)).unwrap(),
                }]
            );
        }
    }

    #[test]
    fn recognizes_adjacent_comparison_operators() {
        assert_eq!(
            scan("<=>=<>").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::LessEqual,
                    range: checked_range(TextSize::new(0), TextSize::new(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::GreaterEqual,
                    range: checked_range(TextSize::new(2), TextSize::new(4)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Less,
                    range: checked_range(TextSize::new(4), TextSize::new(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Greater,
                    range: checked_range(TextSize::new(5), TextSize::new(6)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn comparison_pairs_leave_extra_equals_tokens() {
        assert_eq!(
            scan("<==>==").expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::LessEqual,
                    range: checked_range(TextSize::new(0), TextSize::new(2)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Equal,
                    range: checked_range(TextSize::new(2), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::GreaterEqual,
                    range: checked_range(TextSize::new(3), TextSize::new(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Equal,
                    range: checked_range(TextSize::new(5), TextSize::new(6)).unwrap(),
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
                scan(source).expect("small test source"),
                vec![Token {
                    kind,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
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
                scan(source).expect("small test source"),
                vec![Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
                }]
            );
        }
    }

    #[test]
    fn token_spelling_comes_from_the_source() {
        let source = "print printer";
        let tokens = scan(source).expect("small test source");

        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: SyntaxKind::Print,
                    range: checked_range(TextSize::new(0), TextSize::new(5)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Whitespace,
                    range: checked_range(TextSize::new(5), TextSize::new(6)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(6), TextSize::new(13)).unwrap(),
                },
            ]
        );
        assert_eq!(slice(source, tokens[0].range), Some("print"));
        assert_eq!(slice(source, tokens[1].range), Some(" "));
        assert_eq!(slice(source, tokens[2].range), Some("printer"));
    }

    #[test]
    fn recognizes_number_spellings() {
        for source in ["0", "123", "001", "0.5", "123.456", "00.50"] {
            let tokens = scan(source).expect("small test source");
            assert_eq!(
                tokens,
                vec![Token {
                    kind: SyntaxKind::Number,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
                }]
            );
            assert_eq!(slice(source, tokens[0].range), Some(source));
        }
    }

    #[test]
    fn numbers_leave_dots_signs_and_names_separate() {
        for (source, expected) in [
            (
                "123.",
                vec![(SyntaxKind::Number, 0, 3), (SyntaxKind::Dot, 3, 4)],
            ),
            (
                ".5",
                vec![(SyntaxKind::Dot, 0, 1), (SyntaxKind::Number, 1, 2)],
            ),
            (
                "-12",
                vec![(SyntaxKind::Minus, 0, 1), (SyntaxKind::Number, 1, 3)],
            ),
            (
                "+3.5",
                vec![(SyntaxKind::Plus, 0, 1), (SyntaxKind::Number, 1, 4)],
            ),
            (
                "1.2.3",
                vec![
                    (SyntaxKind::Number, 0, 3),
                    (SyntaxKind::Dot, 3, 4),
                    (SyntaxKind::Number, 4, 5),
                ],
            ),
            (
                "123abc",
                vec![(SyntaxKind::Number, 0, 3), (SyntaxKind::Identifier, 3, 6)],
            ),
            ("a2", vec![(SyntaxKind::Identifier, 0, 2)]),
        ] {
            let tokens = scan(source).expect("small test source");
            assert_eq!(tokens.len(), expected.len(), "{source}");
            for (token, (kind, start, end)) in tokens.iter().zip(expected) {
                assert_eq!(token.kind, kind, "{source}");
                assert_eq!(
                    token.range,
                    checked_range(
                        checked_size(start).expect("representable offset"),
                        checked_size(end).expect("representable offset")
                    )
                    .unwrap(),
                    "{source}"
                );
            }
        }
    }

    #[test]
    fn keeps_complete_string_spellings() {
        for source in [
            r#""""#,
            r#""hello""#,
            r#""// 123 ; print""#,
            "\"é💥\"",
            "\"first\nsecond\"",
            r#""\n""#,
        ] {
            let tokens = scan(source).expect("small test source");
            assert_eq!(
                tokens,
                vec![Token {
                    kind: SyntaxKind::String,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
                }]
            );
            assert_eq!(slice(source, tokens[0].range), Some(source));
        }
    }

    #[test]
    fn strings_stop_at_the_next_quote() {
        let source = r#""a""b";"#;
        let tokens = scan(source).expect("small test source");
        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: SyntaxKind::String,
                    range: checked_range(TextSize::new(0), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::String,
                    range: checked_range(TextSize::new(3), TextSize::new(6)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Semicolon,
                    range: checked_range(TextSize::new(6), TextSize::new(7)).unwrap(),
                },
            ]
        );
        assert_eq!(slice(source, tokens[0].range), Some(r#""a""#));
        assert_eq!(slice(source, tokens[1].range), Some(r#""b""#));
    }

    #[test]
    fn backslash_does_not_escape_a_quote() {
        let source = r#""\"name"#;
        assert_eq!(
            scan(source).expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::String,
                    range: checked_range(TextSize::new(0), TextSize::new(3)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Identifier,
                    range: checked_range(TextSize::new(3), TextSize::new(7)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn unterminated_strings_cover_the_remaining_source() {
        for source in ["\"", "\"é", "\"first\n// still inside"] {
            let tokens = scan(source).expect("small test source");
            assert_eq!(
                tokens,
                vec![Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(
                        TextSize::new(0),
                        checked_size(source.len()).expect("representable offset")
                    )
                    .unwrap(),
                }]
            );
            assert_eq!(slice(source, tokens[0].range), Some(source));
        }

        assert_eq!(
            scan(r#""ok""é"#).expect("small test source"),
            vec![
                Token {
                    kind: SyntaxKind::String,
                    range: checked_range(TextSize::new(0), TextSize::new(4)).unwrap(),
                },
                Token {
                    kind: SyntaxKind::Error,
                    range: checked_range(TextSize::new(4), TextSize::new(7)).unwrap(),
                },
            ]
        );
    }

    #[test]
    fn kind_conversion_accepts_only_declared_kinds() {
        for &kind in SyntaxKind::ALL {
            assert_eq!(SyntaxKind::from_raw(kind.into_raw()), Some(kind));
        }
        assert_eq!(SyntaxKind::from_raw(rowan::SyntaxKind(u16::MAX)), None);
    }
}
