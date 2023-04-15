use phf::phf_map;

use crate::syntax::cursor::Cursor;

use super::cursor::EOF_CHAR;

static KEYWORD: phf::Map<&'static str, TokenKind> = phf_map! {
    "syntax" => TokenKind::SyntaxKw,
    "import" => TokenKind::ImportKw,
    "weak" => TokenKind::WeakKw,
    "public" => TokenKind::PublicKw,
    "package" => TokenKind::PackageKw,
    "option" => TokenKind::OptionKw,
    "inf" => TokenKind::InfKw,
    "repeated" => TokenKind::RepeatedKw,
    "optional" => TokenKind::OptionalKw,
    "required" => TokenKind::RequiredKw,
    "bool" => TokenKind::BoolKw,
    "string" => TokenKind::StringKw,
    "bytes" => TokenKind::BytesKw,
    "float" => TokenKind::FloatKw,
    "double" => TokenKind::DoubleKw,
    "int32" => TokenKind::Int32Kw,
    "int64" => TokenKind::Int64Kw,
    "uint32" => TokenKind::Uint32Kw,
    "uint64" => TokenKind::Uint64Kw,
    "sint32" => TokenKind::Sint32Kw,
    "sint64" => TokenKind::Sint64Kw,
    "fixed32" => TokenKind::Fixed32Kw,
    "fixed64" => TokenKind::Fixed64Kw,
    "sfixed32" => TokenKind::SFixed32Kw,
    "sfixed64" => TokenKind::SFixed64Kw,
    "group" => TokenKind::GroupKw,
    "oneof" => TokenKind::OneofKw,
    "map" => TokenKind::MapKw,
    "extensions" => TokenKind::ExtensionsKw,
    "to" => TokenKind::ToKw,
    "max" => TokenKind::MaxKw,
    "reserved" => TokenKind::ReservedKw,
    "enum" => TokenKind::EnumKw,
    "message" => TokenKind::MessageKw,
    "extend" => TokenKind::ExtendKw,
    "service" => TokenKind::ServiceKw,
    "rpc" => TokenKind::RpcKw,
    "stream" => TokenKind::StreamKw,
    "returns" => TokenKind::ReturnsKw,
};

static OPERATORS: phf::Map<char, TokenKind> = phf_map! {
    ';' => TokenKind::SemiColon,
    ',' => TokenKind::Comma,
    '.' => TokenKind::Dot,
    '/' => TokenKind::Slash,
    ':' => TokenKind::Colon,
    '=' => TokenKind::Equals,
    '-' => TokenKind::Minus,
    '+' => TokenKind::Plus,
    '(' => TokenKind::LParen,
    ')' => TokenKind::RParen,
    '{' => TokenKind::LBrace,
    '}' => TokenKind::RBrace,
    '[' => TokenKind::LBracket,
    ']' => TokenKind::RBracket,
    '<' => TokenKind::LAngle,
    '>' => TokenKind::RAngle,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    IntLiteral,
    FloatLiteral,
    String,
    SemiColon,
    Comma,
    Dot,
    Slash,
    Colon,
    Equals,
    Minus,
    Plus,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LAngle,
    RAngle,
    Identifier,
    SyntaxKw,
    ImportKw,
    WeakKw,
    PublicKw,
    PackageKw,
    OptionKw,
    InfKw,
    RepeatedKw,
    OptionalKw,
    RequiredKw,
    BoolKw,
    StringKw,
    BytesKw,
    FloatKw,
    DoubleKw,
    Int32Kw,
    Int64Kw,
    Uint32Kw,
    Uint64Kw,
    Sint32Kw,
    Sint64Kw,
    Fixed32Kw,
    Fixed64Kw,
    SFixed32Kw,
    SFixed64Kw,
    GroupKw,
    OneofKw,
    MapKw,
    ExtensionsKw,
    ToKw,
    MaxKw,
    ReservedKw,
    EnumKw,
    MessageKw,
    ExtendKw,
    ServiceKw,
    RpcKw,
    StreamKw,
    ReturnsKw,
    LineComment,
    BlockComment,
    NewLine,
    Unknown,
    Eof,
}

impl TokenKind {
    pub fn is_scalar_kw(&self) -> bool {
        matches!(
            self,
            TokenKind::StringKw
                | TokenKind::BoolKw
                | TokenKind::BytesKw
                | TokenKind::FloatKw
                | TokenKind::DoubleKw
                | TokenKind::Int32Kw
                | TokenKind::Int64Kw
                | TokenKind::Uint32Kw
                | TokenKind::Uint64Kw
                | TokenKind::Sint32Kw
                | TokenKind::Sint64Kw
                | TokenKind::Fixed32Kw
                | TokenKind::Fixed64Kw
                | TokenKind::SFixed32Kw
                | TokenKind::SFixed64Kw
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub position: usize,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);

    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.advance_token())
        }
    })
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let pos = self.current_pos();
        let c = match self.bump() {
            Some(c) => c,
            None => EOF_CHAR,
        };

        let token = match c {
            '/' => {
                let next = self.first();
                if next == '/' {
                    self.line_comment()
                } else if next == '*' {
                    self.block_comment()
                } else {
                    Token {
                        value: c.to_string(),
                        kind: TokenKind::Slash,
                        position: pos,
                    }
                }
            }
            c @ '_' => self.identifier_or_keyword(c, pos),
            c if c.is_ascii_alphabetic() => self.identifier_or_keyword(c, pos),
            c if c.is_whitespace() => self.whitespace(c, pos),
            c if c.is_digit(10) => self.numeric_literal(c, pos),
            '.' if self.first().is_digit(10) => self.numeric_literal(c, pos),
            c if OPERATORS.contains_key(&c) => Token {
                value: c.to_string(),
                kind: *OPERATORS.get(&c).unwrap(),
                position: pos,
            },
            c @ '"' | c @ '\'' => self.string(c, pos),
            EOF_CHAR => Token {
                value: c.to_string(),
                kind: TokenKind::Eof,
                position: pos,
            },
            _ => Token {
                value: c.to_string(),
                kind: TokenKind::Unknown,
                position: pos,
            },
        };

        token
    }

    fn whitespace(&mut self, c: char, pos: usize) -> Token {
        let mut value = c.to_string();
        match c {
            '\n' => Token {
                value: c.to_string(),
                kind: TokenKind::NewLine,
                position: pos,
            },
            '\r' => {
                if self.first() == '\n' {
                    let c = self.bump().unwrap();
                    value.push(c);
                    Token {
                        value,
                        kind: TokenKind::NewLine,
                        position: pos,
                    }
                } else {
                    Token {
                        value: c.to_string(),
                        kind: TokenKind::Unknown,
                        position: pos,
                    }
                }
            }
            _ => self.advance_token(),
        }
    }

    fn string(&mut self, c: char, pos: usize) -> Token {
        let mut value = c.to_string();
        let mut next = self.first();

        while next != c {
            // TODO: handle escape sequences
            value.push(self.bump().unwrap());
            next = self.first();
        }

        value.push(self.bump().unwrap());

        Token {
            value,
            kind: TokenKind::String,
            position: pos,
        }
    }

    fn numeric_literal(&mut self, c: char, pos: usize) -> Token {
        let mut value = c.to_string();
        let mut next = self.first();

        while next.is_digit(10)
            || next == '.'
            || next == 'e'
            || next == 'E'
            || next == '+'
            || next == '-'
            || next.is_ascii_alphabetic()
        {
            value.push(self.bump().unwrap());
            next = self.first();
        }

        let mut chars = value.chars();
        let mut kind = TokenKind::Unknown;
        if value.contains(&['.', 'e', 'E']) {
            kind = TokenKind::FloatLiteral;
            let mut has_point = false;
            let mut has_e = false;

            if chars.next().unwrap() == '.' {
                has_point = true;
                let valid = match chars.next() {
                    Some(c) if c.is_digit(10) => true,
                    _ => false,
                };

                if !valid {
                    kind = TokenKind::Unknown;
                    return Token {
                        value,
                        kind,
                        position: pos,
                    };
                }
            }

            while let Some(c) = chars.next() {
                if c == 'e' || c == 'E' {
                    if has_e {
                        kind = TokenKind::Unknown;
                        break;
                    }

                    has_e = true;
                    match chars.next() {
                        Some('+') | Some('-') if chars.next().unwrap().is_digit(10) => {
                            continue;
                        }
                        Some(c) if c.is_digit(10) => {
                            continue;
                        }
                        _ => {
                            kind = TokenKind::Unknown;
                            break;
                        }
                    }
                }

                if c == '.' {
                    if has_point || has_e {
                        kind = TokenKind::Unknown;
                        break;
                    }

                    has_point = true;
                }
            }
        } else {
            if chars.next().unwrap() == '0' {
                match chars.next() {
                    Some('x') | Some('X') if chars.all(|c| c.is_digit(16)) => {
                        kind = TokenKind::IntLiteral
                    }
                    Some(c) if c.is_digit(8) && chars.all(|c| c.is_digit(8)) => {
                        kind = TokenKind::IntLiteral
                    }

                    Some(_) => kind = TokenKind::Unknown,
                    None => kind = TokenKind::IntLiteral,
                }
            } else {
                kind = TokenKind::IntLiteral;
            }
        }

        Token {
            value,
            kind,
            position: pos,
        }
    }

    fn identifier_or_keyword(&mut self, c: char, pos: usize) -> Token {
        let mut value = c.to_string();

        loop {
            match self.first() {
                c if c.is_ascii_alphanumeric() => value.push(self.bump().unwrap()),
                '_' => value.push(self.bump().unwrap()),
                _ => break,
            }
        }

        if let Some(kind) = KEYWORD.get(&value) {
            Token {
                value,
                kind: *kind,
                position: pos,
            }
        } else {
            Token {
                value,
                kind: TokenKind::Identifier,
                position: pos,
            }
        }
    }

    fn line_comment(&mut self) -> Token {
        todo!()
    }

    fn block_comment(&mut self) -> Token {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token, TokenKind};

    #[test]
    fn double_quoted_string() {
        let input = r#""hello""#;

        let expected_tokens = vec![Token {
            value: r#""hello""#.to_string(),
            kind: TokenKind::String,
            position: 0,
        }];

        let actual_tokens: Vec<Token> = tokenize(input).collect();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        expected_tokens
            .iter()
            .zip(actual_tokens)
            .for_each(|(e, a)| {
                assert_eq!(e, &a);
            });
    }

    #[test]
    fn new_lines() {
        let inputs = vec![
            ("\r\n", TokenKind::NewLine),
            ("\n", TokenKind::NewLine),
            ("\r", TokenKind::Unknown),
        ];

        for (input, expected_kind) in inputs {
            let expected_tokens = vec![Token {
                value: input.to_string(),
                kind: expected_kind,
                position: 0,
            }];

            let actual_tokens: Vec<Token> = tokenize(input).collect();

            assert_eq!(expected_tokens.len(), actual_tokens.len());

            expected_tokens
                .iter()
                .zip(actual_tokens)
                .for_each(|(e, a)| {
                    assert_eq!(e, &a);
                });
        }
    }

    #[test]
    fn numeric_literal() {
        let inputs = vec![
            ("0", TokenKind::IntLiteral),
            ("1234", TokenKind::IntLiteral),
            ("0741", TokenKind::IntLiteral),
            ("0781", TokenKind::Unknown),
            ("0x0f6db2", TokenKind::IntLiteral),
            ("0X0f6db2", TokenKind::IntLiteral),
            ("0.0", TokenKind::FloatLiteral),
            ("1.", TokenKind::FloatLiteral),
            (".123", TokenKind::FloatLiteral),
            ("555.555", TokenKind::FloatLiteral),
            ("1.234e-12", TokenKind::FloatLiteral),
            (".953e20", TokenKind::FloatLiteral),
            ("5E+40", TokenKind::FloatLiteral),
        ];

        for (input, expected_kind) in inputs {
            let expected_tokens = vec![Token {
                value: input.to_string(),
                kind: expected_kind,
                position: 0,
            }];

            let actual_tokens: Vec<Token> = tokenize(input).collect();

            assert_eq!(
                expected_tokens.len(),
                actual_tokens.len(),
                "expected = {:?}, actual = {:?}",
                expected_tokens,
                actual_tokens
            );

            expected_tokens
                .iter()
                .zip(actual_tokens)
                .for_each(|(e, a)| {
                    assert_eq!(e, &a);
                });
        }
    }

    #[test]
    pub fn keywords() {
        let input = vec![
            ("syntax", TokenKind::SyntaxKw),
            ("import", TokenKind::ImportKw),
            ("package", TokenKind::PackageKw),
            ("option", TokenKind::OptionKw),
            ("message", TokenKind::MessageKw),
            ("enum", TokenKind::EnumKw),
            ("service", TokenKind::ServiceKw),
            ("rpc", TokenKind::RpcKw),
            ("returns", TokenKind::ReturnsKw),
            ("extend", TokenKind::ExtendKw),
            ("extensions", TokenKind::ExtensionsKw),
            ("reserved", TokenKind::ReservedKw),
            ("to", TokenKind::ToKw),
            ("max", TokenKind::MaxKw),
            ("weak", TokenKind::WeakKw),
            ("repeated", TokenKind::RepeatedKw),
            ("map", TokenKind::MapKw),
            ("oneof", TokenKind::OneofKw),
            ("group", TokenKind::GroupKw),
            ("required", TokenKind::RequiredKw),
            ("optional", TokenKind::OptionalKw),
            ("double", TokenKind::DoubleKw),
            ("float", TokenKind::FloatKw),
            ("int32", TokenKind::Int32Kw),
            ("int64", TokenKind::Int64Kw),
            ("uint32", TokenKind::Uint32Kw),
            ("uint64", TokenKind::Uint64Kw),
            ("sint32", TokenKind::Sint32Kw),
            ("sint64", TokenKind::Sint64Kw),
            ("fixed32", TokenKind::Fixed32Kw),
            ("fixed64", TokenKind::Fixed64Kw),
            ("sfixed32", TokenKind::SFixed32Kw),
            ("sfixed64", TokenKind::SFixed64Kw),
            ("bool", TokenKind::BoolKw),
            ("string", TokenKind::StringKw),
            ("bytes", TokenKind::BytesKw),
            ("stream", TokenKind::StreamKw),
            ("inf", TokenKind::InfKw),
            ("public", TokenKind::PublicKw),
        ];

        for (input, expected_kind) in input {
            let expected_tokens = vec![Token {
                value: input.to_string(),
                kind: expected_kind,
                position: 0,
            }];

            let actual_tokens: Vec<Token> = tokenize(input).collect();

            assert_eq!(
                expected_tokens.len(),
                actual_tokens.len(),
                "expected = {:?}, actual = {:?}",
                expected_tokens,
                actual_tokens
            );

            expected_tokens
                .iter()
                .zip(actual_tokens)
                .for_each(|(e, a)| {
                    assert_eq!(e, &a);
                });
        }
    }

    #[test]
    fn operators() {
        let input = vec![
            (';', TokenKind::SemiColon),
            (',', TokenKind::Comma),
            ('=', TokenKind::Equals),
            ('{', TokenKind::LBrace),
            ('}', TokenKind::RBrace),
            ('[', TokenKind::LBracket),
            (']', TokenKind::RBracket),
            ('(', TokenKind::LParen),
            (')', TokenKind::RParen),
            ('<', TokenKind::LAngle),
            ('>', TokenKind::RAngle),
            (':', TokenKind::Colon),
            ('.', TokenKind::Dot),
            ('+', TokenKind::Plus),
            ('-', TokenKind::Minus),
            ('/', TokenKind::Slash),
        ];

        for (input, expected_kind) in input {
            let expected_tokens = vec![Token {
                value: input.to_string(),
                kind: expected_kind,
                position: 0,
            }];

            let actual_tokens: Vec<Token> = tokenize(&input.to_string()).collect();

            assert_eq!(
                expected_tokens.len(),
                actual_tokens.len(),
                "expected = {:?}, actual = {:?}",
                expected_tokens,
                actual_tokens
            );

            expected_tokens
                .iter()
                .zip(actual_tokens)
                .for_each(|(e, a)| {
                    assert_eq!(e, &a);
                });
        }
    }

    #[test]
    fn combinations() {
        let input: &[(&str, &[Token])] = &[
            (
                "syntax = \"proto3\";",
                &[
                    Token {
                        value: "syntax".to_string(),
                        kind: TokenKind::SyntaxKw,
                        position: 0,
                    },
                    Token {
                        value: "=".to_string(),
                        kind: TokenKind::Equals,
                        position: 7,
                    },
                    Token {
                        value: "\"proto3\"".to_string(),
                        kind: TokenKind::String,
                        position: 9,
                    },
                    Token {
                        value: ";".to_string(),
                        kind: TokenKind::SemiColon,
                        position: 17,
                    },
                ],
            ),
            (
                "message Foo {",
                &[
                    Token {
                        value: "message".to_string(),
                        kind: TokenKind::MessageKw,
                        position: 0,
                    },
                    Token {
                        value: "Foo".to_string(),
                        kind: TokenKind::Identifier,
                        position: 8,
                    },
                    Token {
                        value: "{".to_string(),
                        kind: TokenKind::LBrace,
                        position: 12,
                    },
                ],
            ),
            (
                "message Foo { optional int32 bar = 1; }",
                &[
                    Token {
                        value: "message".to_string(),
                        kind: TokenKind::MessageKw,
                        position: 0,
                    },
                    Token {
                        value: "Foo".to_string(),
                        kind: TokenKind::Identifier,
                        position: 8,
                    },
                    Token {
                        value: "{".to_string(),
                        kind: TokenKind::LBrace,
                        position: 12,
                    },
                    Token {
                        value: "optional".to_string(),
                        kind: TokenKind::OptionalKw,
                        position: 14,
                    },
                    Token {
                        value: "int32".to_string(),
                        kind: TokenKind::Int32Kw,
                        position: 23,
                    },
                    Token {
                        value: "bar".to_string(),
                        kind: TokenKind::Identifier,
                        position: 29,
                    },
                    Token {
                        value: "=".to_string(),
                        kind: TokenKind::Equals,
                        position: 33,
                    },
                    Token {
                        value: "1".to_string(),
                        kind: TokenKind::IntLiteral,
                        position: 35,
                    },
                    Token {
                        value: ";".to_string(),
                        kind: TokenKind::SemiColon,
                        position: 36,
                    },
                    Token {
                        value: "}".to_string(),
                        kind: TokenKind::RBrace,
                        position: 38,
                    },
                ],
            ),
            (
                "message Foo { optional int32 bar = 1; optional int32 baz = 2; }",
                &[
                    Token {
                        value: "message".to_string(),
                        kind: TokenKind::MessageKw,
                        position: 0,
                    },
                    Token {
                        value: "Foo".to_string(),
                        kind: TokenKind::Identifier,
                        position: 8,
                    },
                    Token {
                        value: "{".to_string(),
                        kind: TokenKind::LBrace,
                        position: 12,
                    },
                    Token {
                        value: "optional".to_string(),
                        kind: TokenKind::OptionalKw,
                        position: 14,
                    },
                    Token {
                        value: "int32".to_string(),
                        kind: TokenKind::Int32Kw,
                        position: 23,
                    },
                    Token {
                        value: "bar".to_string(),
                        kind: TokenKind::Identifier,
                        position: 29,
                    },
                    Token {
                        value: "=".to_string(),
                        kind: TokenKind::Equals,
                        position: 33,
                    },
                    Token {
                        value: "1".to_string(),
                        kind: TokenKind::IntLiteral,
                        position: 35,
                    },
                    Token {
                        value: ";".to_string(),
                        kind: TokenKind::SemiColon,
                        position: 36,
                    },
                    Token {
                        value: "optional".to_string(),
                        kind: TokenKind::OptionalKw,
                        position: 38,
                    },
                    Token {
                        value: "int32".to_string(),
                        kind: TokenKind::Int32Kw,
                        position: 47,
                    },
                    Token {
                        value: "baz".to_string(),
                        kind: TokenKind::Identifier,
                        position: 53,
                    },
                    Token {
                        value: "=".to_string(),
                        kind: TokenKind::Equals,
                        position: 57,
                    },
                    Token {
                        value: "2".to_string(),
                        kind: TokenKind::IntLiteral,
                        position: 59,
                    },
                    Token {
                        value: ";".to_string(),
                        kind: TokenKind::SemiColon,
                        position: 60,
                    },
                    Token {
                        value: "}".to_string(),
                        kind: TokenKind::RBrace,
                        position: 62,
                    },
                ],
            ),
        ];

        for (input, expected_tokens) in input {
            let actual_tokens: Vec<Token> = tokenize(&input.to_string()).collect();

            assert_eq!(
                expected_tokens.len(),
                actual_tokens.len(),
                "expected = {:?}, actual = {:?}",
                expected_tokens,
                actual_tokens
            );

            expected_tokens
                .iter()
                .zip(actual_tokens)
                .for_each(|(e, a)| {
                    assert_eq!(e, &a);
                });
        }
    }
}
