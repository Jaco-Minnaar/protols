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
    DecimalIntLiteral,
    OctalIntLiteral,
    HexIntLiteral,
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
    Divide,
    LineComment,
    BlockComment,
    NewLine,
    Unknown,
}

pub struct Token {
    value: String,
    kind: TokenKind,
    position: usize,
}

pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
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
        let c = match self.bump() {
            Some(c) => c,
            None => EOF_CHAR,
        };
        let pos = self.current_pos();

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
                        kind: TokenKind::Divide,
                        position: pos,
                    }
                }
            }
            c if OPERATORS.contains_key(&c) => Token {
                value: c.to_string(),
                kind: *OPERATORS.get(&c).unwrap(),
                position: pos,
            },
            c @ '_' => self.identifier_or_keyword(c, pos),
            c if c.is_ascii_alphabetic() => self.identifier_or_keyword(c, pos),
            c if c.is_whitespace() => {
                if c == '\n' || (c == '\r' && self.first() != '\n') {
                    Token {
                        value: c.to_string(),
                        kind: TokenKind::NewLine,
                        position: pos,
                    }
                } else {
                    self.advance_token()
                }
            }
            c if c.is_digit(10) => self.numeric_literal(c, pos),
            '.' if self.first().is_digit(10) => self.numeric_literal(c, pos),
            '"' => self.string(c, pos),
            _ => Token {
                value: c.to_string(),
                kind: TokenKind::Unknown,
                position: pos,
            },
        };

        token
    }

    fn string(&mut self, c: char, pos: usize) -> Token {
        let mut value = c.to_string();
        let mut next = self.first();

        while next != '"' {
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
            || next == 'i'
            || next.is_ascii_alphabetic()
        {
            value.push(self.bump().unwrap());
            next = self.first();
        }

        let mut chars = value.chars();
        let mut kind = TokenKind::Unknown;
        if value.contains('.') {
            let mut has_point = false;
            let mut has_e = false;

            if chars.next().unwrap() == '.' {
                has_point = true;
                let valid = match chars.next() {
                    Some(c) if c.is_digit(10) => true,
                    _ => false,
                };

                if !valid {
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
                    let c = chars.next().unwrap();
                    match c {
                        '+' | '-' if chars.next().unwrap().is_digit(10) => {
                            kind = TokenKind::FloatLiteral;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            kind = TokenKind::FloatLiteral;
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
                        kind = TokenKind::HexIntLiteral
                    }
                    Some(c) if c.is_digit(8) && chars.all(|c| c.is_digit(8)) => {
                        kind = TokenKind::OctalIntLiteral
                    }

                    Some(_) => kind = TokenKind::Unknown,
                    None => kind = TokenKind::DecimalIntLiteral,
                }
            } else {
                kind = TokenKind::DecimalIntLiteral;
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
        while self.first().is_ascii_alphabetic() {
            value.push(self.bump().unwrap());
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
