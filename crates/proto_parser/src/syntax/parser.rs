use std::{iter::Peekable, str::FromStr};

use crate::syntax::ast::{
    message::{MessageNode, OneofField},
    option::OptionNamePart,
    EnumNode, ExtensionNode, TypeName,
};

use super::{
    ast::{
        message::{
            FieldCardinality, FieldDeclaration, FieldType, MapFieldDeclaration, MessageElement,
            OneofDeclaration, OneofElement,
        },
        option::{OptionName, OptionNode, OptionValue},
        service::{MethodNode, ServiceNode},
        EnumElement, ExtensionElement, ImportModifier, ImportNode, Node, PackageNode, Reserved,
        Root, RootNode, ScalarType, SyntaxNode, SyntaxType, TagEnd, TagRange,
    },
    lexer::{Token, TokenKind},
};

pub struct ParseResult {
    pub root: Root,
    pub errors: Vec<ParseError>,
}

pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl ParseError {
    pub fn new(message: String, position: usize) -> Self {
        Self { message, position }
    }
}

type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
    errors: Vec<ParseError>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self, file_name: &str) -> ParseResult {
        let mut root = Root::new(file_name.to_string());

        while !self.is_at_end() {
            match self.root_node() {
                Ok(node) => root.add_node(node),
                Err(err) => {
                    self.errors.push(err);
                    self.sync();
                }
            }
        }

        ParseResult {
            root,
            errors: self.errors,
        }
    }

    fn is_at_end(&mut self) -> bool {
        match self.peek_kind() {
            Some(TokenKind::Eof) | None => true,
            _ => false,
        }
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<Token> {
        if let Some(t) = self.peek_kind() {
            if *t == token_kind {
                return Ok(self.advance().unwrap());
            }
        }

        Err(ParseError::new(
            format!("Expected token: {:?}", token_kind),
            self.tokens.peek().unwrap().position,
        ))
    }

    fn peek_kind(&mut self) -> Option<&TokenKind> {
        if let Some(t) = self.tokens.peek() {
            Some(&t.kind)
        } else {
            None
        }
    }

    fn sync(&mut self) {
        loop {
            match self.advance() {
                Some(Token {
                    kind: TokenKind::RBrace | TokenKind::SemiColon,
                    ..
                })
                | None => return,
                _ => {}
            };
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn root_node(&mut self) -> Result<RootNode> {
        let result = match self.peek_kind() {
            Some(TokenKind::SyntaxKw) => RootNode::SyntaxDeclaration(self.syntax_node()?),
            Some(TokenKind::PackageKw) => RootNode::PackageDeclaration(self.package_node()?),
            Some(TokenKind::ImportKw) => RootNode::ImportDeclaration(self.import_node()?),
            Some(TokenKind::OptionKw) => RootNode::OptionDeclaration(self.option_node()?),
            Some(TokenKind::MessageKw) => RootNode::MessageDeclaration(self.message_node()?),
            Some(TokenKind::EnumKw) => RootNode::EnumDeclaration(self.enum_node()?),
            Some(TokenKind::ServiceKw) => RootNode::ServiceDeclaration(self.service_node()?),
            Some(TokenKind::ExtendKw) => RootNode::ExtensionDeclaration(self.extend_node()?),
            _ => {
                let token = self.advance().unwrap();
                let err = ParseError::new(
                    format!("Unexpected token: {:?}", token.kind),
                    token.position,
                );
                return Err(err);
            }
        };

        Ok(result)
    }

    fn syntax_node(&mut self) -> Result<Node<SyntaxNode>> {
        let start = self.advance().unwrap().position;

        self.expect(TokenKind::Equals)?;

        let token_kind = if let Some(TokenKind::StringKw) = self.peek_kind() {
            let token = self.advance().unwrap();
            match token.value.as_str() {
                "proto2" => SyntaxType::Proto2,
                "proto3" => SyntaxType::Proto3,
                _ => {
                    let err = ParseError::new(
                        format!("Invalid syntax version: {:?}", token.kind),
                        token.position,
                    );
                    return Err(err);
                }
            }
        } else {
            let err = ParseError::new(
                format!("Expected string after '='",),
                self.tokens.peek().unwrap().position,
            );
            return Err(err);
        };

        let end_token = self.expect(TokenKind::SemiColon)?;

        let syntax_node = Node::new(
            SyntaxNode {
                proto_type: token_kind,
            },
            start,
            end_token.position,
        );
        Ok(syntax_node)
    }

    fn package_node(&mut self) -> Result<Node<PackageNode>> {
        let name = self.qualified_identifier()?;

        let end_token = self.expect(TokenKind::SemiColon)?;

        self.advance().unwrap();

        let package_node = Node::new(
            PackageNode {
                package_name: name.value,
            },
            name.start,
            end_token.position,
        );
        Ok(package_node)
    }

    fn import_node(&mut self) -> Result<Node<ImportNode>> {
        let start = self.advance().unwrap().position;

        let modifier = match self.peek_kind() {
            Some(TokenKind::PublicKw) => {
                self.advance().unwrap();
                Some(ImportModifier::Public)
            }
            Some(TokenKind::WeakKw) => {
                self.advance().unwrap();
                Some(ImportModifier::Weak)
            }
            _ => None,
        };

        let file_name = self.string()?;

        let end_token = self.expect(TokenKind::SemiColon)?;

        let import_node = Node::new(
            ImportNode {
                modifier,
                package_name: file_name.value,
            },
            start,
            end_token.position,
        );

        Ok(import_node)
    }

    fn option_node(&mut self) -> Result<Node<OptionNode>> {
        let start = self.advance().unwrap().position;

        let option_name = self.option_name()?;

        self.expect(TokenKind::Equals)?;

        let option_value = self.option_value()?;

        let end_token = self.expect(TokenKind::SemiColon)?;

        let option_node = Node::new(
            OptionNode {
                name: option_name,
                value: option_value,
            },
            start,
            end_token.position,
        );

        Ok(option_node)
    }

    fn message_node(&mut self) -> Result<Node<MessageNode>> {
        let start = self.advance().unwrap().position;

        let identifier = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LBrace)?;

        let mut elements = Vec::new();
        while !matches!(self.peek_kind(), Some(TokenKind::RBrace) | None) {
            let element = self.message_element()?;
            elements.push(element);
        }

        let end_token = self.expect(TokenKind::RBrace)?;

        let message_node = Node::new(
            MessageNode {
                name: identifier.value,
                elements,
            },
            start,
            end_token.position,
        );

        Ok(message_node)
    }

    fn enum_node(&mut self) -> Result<Node<EnumNode>> {
        let start = self.advance().unwrap().position;

        let identifier_token = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LBrace)?;

        let elements = self.enum_elements()?;

        let end_token = self.expect(TokenKind::RBrace)?;

        let enum_node = Node::new(
            EnumNode {
                name: identifier_token.value,
                elements,
            },
            start,
            end_token.position,
        );

        Ok(enum_node)
    }

    fn service_node(&mut self) -> Result<Node<ServiceNode>> {
        let start = self.advance().unwrap().position;

        let identifier_token = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LBrace)?;

        let elements = self.service_elements()?;

        let end_token = self.expect(TokenKind::RBrace)?;

        let service_node = Node::new(
            ServiceNode {
                name: identifier_token.value,
                methods: elements,
            },
            start,
            end_token.position,
        );

        Ok(service_node)
    }

    fn extend_node(&mut self) -> Result<Node<ExtensionNode>> {
        let identifier_token = self.qualified_identifier()?;

        self.expect(TokenKind::LBrace)?;

        let elements = self.extension_elements()?;

        let end_token = self.expect(TokenKind::RBrace)?;

        let extend_node = Node::new(
            ExtensionNode {
                extendee: identifier_token.value,
                elements,
            },
            identifier_token.start,
            end_token.position,
        );

        Ok(extend_node)
    }

    fn string(&mut self) -> Result<Node<String>> {
        let mut token = self.expect(TokenKind::String)?;

        let mut string = String::from_str(token.value.as_str()).unwrap();
        let start = token.position;
        let mut end = token.position + token.value.len();
        while let Some(TokenKind::String) = self.peek_kind() {
            token = self.advance().unwrap();
            string.push_str(token.value.as_str());
            end = token.position + token.value.len();
        }

        let string = format!("\"{}\"", string.replace("\"", ""));

        let node = Node::new(string, start, end);

        Ok(node)
    }

    fn qualified_identifier(&mut self) -> Result<Node<Vec<String>>> {
        let identifier = self.expect(TokenKind::Identifier)?;
        let start = identifier.position;

        let mut identifiers = vec![identifier.value];

        while let Some(TokenKind::Dot) = self.peek_kind() {
            self.advance().unwrap();
            let identifier = self.expect(TokenKind::Identifier)?;

            identifiers.push(identifier.value);
        }

        let end = self.tokens.peek().unwrap().position;
        let node = Node::new(identifiers, start, end);
        Ok(node)
    }

    fn option_name(&mut self) -> Result<Node<OptionName>> {
        let mut name = Vec::new();

        loop {
            match self.peek_kind() {
                Some(TokenKind::Identifier) => {
                    let identifier = self.advance().unwrap();
                    let part = Node::new(
                        OptionNamePart::SimpleName(identifier.value),
                        identifier.position,
                        identifier.position,
                    );
                    name.push(part);
                }
                Some(TokenKind::LParen) => {
                    let identifier = self.qualified_identifier()?;

                    let part = Node::new(
                        OptionNamePart::ExtensionName(TypeName::from(identifier.value)),
                        identifier.start,
                        identifier.end,
                    );

                    name.push(part);
                }
                _ => {
                    break;
                }
            }
        }

        if name.is_empty() {
            return Err(ParseError::new(
                format!("Expected option name",),
                self.tokens.peek().unwrap().position,
            ));
        }

        let start = name.first().unwrap().start;
        let end = name.last().unwrap().end;

        Ok(Node::new(name, start, end))
    }

    fn option_value(&mut self) -> Result<Node<OptionValue>> {
        match self.peek_kind() {
            Some(TokenKind::String) => {
                let token = self.advance().unwrap();
                let end = token.position + token.value.len();
                let value = Node::new(OptionValue::StringLiteral(token.value), token.position, end);
                Ok(value)
            }
            Some(TokenKind::Minus) => {
                let start = self.advance().unwrap().position;
                match self.peek_kind() {
                    Some(TokenKind::IntLiteral) => {
                        let token = self.advance().unwrap();
                        let end = start + token.value.len();
                        let value = Node::new(
                            OptionValue::IntLiteral(format!("-{}", token.value)),
                            start,
                            end,
                        );
                        Ok(value)
                    }
                    Some(TokenKind::FloatLiteral) => {
                        let token = self.advance().unwrap();
                        let end = start + token.value.len();
                        let value = Node::new(
                            OptionValue::FloatLiteral(format!("-{}", token.value)),
                            start,
                            end,
                        );
                        Ok(value)
                    }
                    _ => {
                        return Err(ParseError::new(
                            format!("Expected int or float literal"),
                            start,
                        ))
                    }
                }
            }
            Some(TokenKind::Plus) => {
                let start = self.advance().unwrap().position;

                match self.peek_kind() {
                    Some(TokenKind::IntLiteral) => {
                        let token = self.advance().unwrap();
                        let end = start + token.value.len();
                        let value = Node::new(
                            OptionValue::UintLiteral(format!("+{}", token.value)),
                            start,
                            end,
                        );
                        Ok(value)
                    }
                    Some(TokenKind::FloatLiteral) => {
                        let token = self.advance().unwrap();
                        let end = start + token.value.len();
                        let value = Node::new(
                            OptionValue::FloatLiteral(format!("+{}", token.value)),
                            start,
                            end,
                        );
                        Ok(value)
                    }
                    _ => {
                        return Err(ParseError::new(
                            format!("Expected int or float literal"),
                            start,
                        ))
                    }
                }
            }
            Some(TokenKind::IntLiteral) => {
                let token = self.advance().unwrap();
                let end = token.position + token.value.len();
                let value = Node::new(OptionValue::UintLiteral(token.value), token.position, end);
                Ok(value)
            }
            Some(TokenKind::FloatLiteral) => {
                let token = self.advance().unwrap();
                let end = token.position + token.value.len();
                let value = Node::new(OptionValue::FloatLiteral(token.value), token.position, end);
                Ok(value)
            }
            _ => {
                return Err(ParseError::new(
                    format!("Expected option value",),
                    self.tokens.peek().unwrap().position,
                ));
            }
        }
    }

    fn message_element(&mut self) -> Result<Node<MessageElement>> {
        let element = match self.peek_kind() {
            Some(
                TokenKind::RequiredKw
                | TokenKind::OptionalKw
                | TokenKind::RepeatedKw
                | TokenKind::Dot
                | TokenKind::Identifier,
            ) => {
                let decl = self.field_decl()?;
                Node::new(
                    MessageElement::FieldDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(t) if t.is_scalar_kw() => {
                let decl = self.field_decl()?;
                Node::new(
                    MessageElement::FieldDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::EnumKw) => {
                let decl = self.enum_node()?;
                Node::new(
                    MessageElement::EnumDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::MessageKw) => {
                let decl = self.message_node()?;
                Node::new(
                    MessageElement::MessageDeclaration(Box::new(decl.value)),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::OneofKw) => {
                let decl = self.oneof_node()?;
                Node::new(
                    MessageElement::OneOfDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::MapKw) => {
                let decl = self.map_field_decl()?;
                Node::new(
                    MessageElement::MapFieldDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::ExtensionsKw) => {
                let decl = self.extend_node()?;
                Node::new(
                    MessageElement::ExtensionDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::ReservedKw) => {
                let decl = self.reserved_node()?;
                Node::new(
                    MessageElement::ReservedDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            Some(TokenKind::OptionKw) => {
                let decl = self.option_node()?;
                Node::new(
                    MessageElement::OptionDeclaration(decl.value),
                    decl.start,
                    decl.end,
                )
            }
            _ => todo!("unknown message element"),
        };

        Ok(element)
    }

    fn field_decl(&mut self) -> Result<Node<FieldDeclaration>> {
        let mut cardinality = self.field_cardinality()?;

        let field_type = self.field_type()?;

        if cardinality.start == 0 {
            cardinality.start = field_type.start;
            cardinality.end = field_type.start;
        }

        let start = cardinality.start;

        let field_name = self.field_name()?;

        self.expect(TokenKind::Equals)?;

        let field_number = self.expect(TokenKind::IntLiteral)?;
        let field_number = Node::new(
            field_number.value.parse::<u32>().unwrap(),
            field_number.position,
            field_number.position + field_number.value.len(),
        );

        let options = self.compact_options()?;

        let end = self.expect(TokenKind::SemiColon)?.position;

        let decl = FieldDeclaration {
            cardinality,
            type_name: field_type,
            name: field_name,
            number: field_number,
            options,
        };

        Ok(Node::new(decl, start, end))
    }

    fn field_cardinality(&mut self) -> Result<Node<FieldCardinality>> {
        let cardinality = match self.peek_kind() {
            Some(TokenKind::RequiredKw) => {
                let token = self.advance().unwrap();
                let end = token.position + token.value.len();
                Node::new(FieldCardinality::Required, token.position, end)
            }
            Some(TokenKind::OptionalKw) => {
                let token = self.advance().unwrap();
                let end = token.position + token.value.len();
                Node::new(FieldCardinality::Optional, token.position, end)
            }
            Some(TokenKind::RepeatedKw) => {
                let token = self.advance().unwrap();
                let end = token.position + token.value.len();
                Node::new(FieldCardinality::Repeated, token.position, end)
            }
            Some(t) if t.is_scalar_kw() => Node::new(FieldCardinality::Optional, 0, 0),
            Some(TokenKind::Dot) => Node::new(FieldCardinality::Optional, 0, 0),
            Some(TokenKind::Identifier) => Node::new(FieldCardinality::Optional, 0, 0),
            _ => {
                return Err(ParseError::new(
                    format!("Expected field cardinality"),
                    self.tokens.peek().unwrap().position,
                ));
            }
        };

        Ok(cardinality)
    }

    fn field_type(&mut self) -> Result<Node<FieldType>> {
        let field_type = match self.peek_kind() {
            Some(TokenKind::Identifier | TokenKind::Dot) => {
                let name = self.type_name()?;
                Node::new(FieldType::TypeName(name.value), name.start, name.end)
            }
            Some(t) if t.is_scalar_kw() => {
                let token = self.advance().unwrap();
                let scalar_type = ScalarType::try_from(token.kind).unwrap();

                Node::new(
                    FieldType::ScalarType(scalar_type),
                    token.position,
                    token.position + token.value.len(),
                )
            }
            _ => {
                return Err(ParseError::new(
                    format!("Expected field type"),
                    self.tokens.peek().unwrap().position,
                ));
            }
        };

        Ok(field_type)
    }

    fn field_name(&mut self) -> Result<Node<String>> {
        let token = self.expect(TokenKind::Identifier)?;
        let start = token.position;
        let end = token.position + token.value.len();
        Ok(Node::new(token.value, start, end))
    }

    fn enum_elements(&self) -> Result<Vec<Node<EnumElement>>> {
        todo!()
    }

    fn service_elements(&self) -> Result<Vec<Node<MethodNode>>> {
        todo!()
    }

    fn extension_elements(&self) -> Result<Vec<Node<ExtensionElement>>> {
        todo!()
    }

    fn type_name(&mut self) -> Result<Node<TypeName>> {
        let absolute = if let Some(TokenKind::Dot) = self.peek_kind() {
            self.advance().unwrap();
            true
        } else {
            false
        };

        let parts = self.qualified_identifier()?;

        let start = if absolute {
            parts.start - 1
        } else {
            parts.start
        };

        let end = parts.end;

        let name = TypeName {
            absolute,
            parts: parts.value,
        };

        Ok(Node::new(name, start, end))
    }

    fn reserved_node(&mut self) -> Result<Node<Reserved>> {
        let start = self.expect(TokenKind::ReservedKw)?.position;

        match self.peek_kind() {
            Some(TokenKind::IntLiteral) => {
                let range = self.tag_range()?;
                let end = range.last().unwrap().end;
                Ok(Node::new(Reserved::TagRanges(range), start, end))
            }
            Some(TokenKind::String) => {
                let names = self.reserved_names()?;
                let end = names.last().unwrap().end;
                Ok(Node::new(Reserved::Names(names), start, end))
            }
            _ => {
                return Err(ParseError::new(
                    format!("Expected reserved range or names"),
                    self.tokens.peek().unwrap().position,
                ));
            }
        }
    }

    fn map_field_decl(&self) -> Result<Node<MapFieldDeclaration>> {
        todo!()
    }

    fn oneof_node(&mut self) -> Result<Node<OneofDeclaration>> {
        let start = self.expect(TokenKind::OneofKw)?.position;
        let name = self.expect(TokenKind::Identifier)?;
        let name_start = name.position;
        let name_end = name.position + name.value.len();
        let name = Node::new(name.value, name_start, name_end);

        self.expect(TokenKind::LBrace)?;

        let mut elements = Vec::new();

        while !matches!(self.peek_kind(), Some(TokenKind::RBrace)) {
            let element = self.oneof_element()?;
            elements.push(element);
        }

        let end = self.expect(TokenKind::RBrace)?;

        let decl = OneofDeclaration { name, elements };

        Ok(Node::new(decl, start, end.position))
    }

    fn oneof_element(&mut self) -> Result<Node<OneofElement>> {
        match self.peek_kind() {
            Some(TokenKind::OptionKw) => {
                let option = self.option_node()?;
                Ok(Node::new(
                    OneofElement::Option(option.value),
                    option.start,
                    option.end,
                ))
            }
            Some(TokenKind::Identifier) | Some(TokenKind::Dot) => {
                let element_type = self.type_name()?;
                let start = element_type.start;
                let name = self.field_name()?;
                self.expect(TokenKind::Equals)?;
                let number = self.expect(TokenKind::IntLiteral)?;
                let number = Node::new(
                    number.value.parse::<u32>().unwrap(),
                    number.position,
                    number.position + number.value.len(),
                );
                let options = if let Some(TokenKind::RBrace) = self.peek_kind() {
                    Some(self.compact_options()?)
                } else {
                    None
                };

                let end = self.expect(TokenKind::SemiColon)?.position;

                let element = OneofElement::OneofField(OneofField {
                    type_name: element_type,
                    name,
                    number,
                    options,
                });

                Ok(Node::new(element, start, end))
            }
            _ => {
                let err = ParseError::new(
                    format!("Expected oneof element"),
                    self.tokens.peek().unwrap().position,
                );

                return Err(err);
            }
        }
    }

    fn compact_options(&mut self) -> Result<Vec<Node<OptionNode>>> {
        self.expect(TokenKind::LBracket)?;

        let mut options = Vec::new();

        while !matches!(self.peek_kind(), Some(TokenKind::RBracket)) {
            let option = self.option_node()?;
            options.push(option);

            if let Some(TokenKind::Comma) = self.peek_kind() {
                self.advance().unwrap();
            } else {
                break;
            }
        }

        Ok(options)
    }

    fn tag_range(&mut self) -> Result<Vec<Node<TagRange>>> {
        let mut ranges = Vec::new();

        loop {
            let start = self.expect(TokenKind::IntLiteral)?;
            let end = if let Some(TokenKind::ToKw) = self.peek_kind() {
                self.advance().unwrap();

                let end = match self.advance() {
                    Some(Token {
                        kind: TokenKind::IntLiteral,
                        position,
                        value,
                    }) => Node::new(
                        TagEnd::Tag(value.parse::<u32>().unwrap()),
                        position,
                        position + value.len(),
                    ),
                    Some(Token {
                        kind: TokenKind::Identifier,
                        value,
                        position,
                    }) if &value == "max" => {
                        Node::new(TagEnd::Max, position, position + value.len())
                    }
                    _ => {
                        return Err(ParseError::new(
                            format!("Expected tag end"),
                            self.tokens.peek().unwrap().position,
                        ));
                    }
                };

                Some(end)
            } else {
                None
            };

            let range = TagRange {
                start: Node::new(
                    start.value.parse::<u32>().unwrap(),
                    start.position,
                    start.position + start.value.len(),
                ),
                end,
            };
            let start = range.start.start;
            let end = range.end.as_ref().map(|n| n.end).unwrap_or(range.start.end);

            ranges.push(Node::new(range, start, end));

            if let Some(TokenKind::Comma) = self.peek_kind() {
                self.advance().unwrap();
            } else {
                break;
            }
        }

        Ok(ranges)
    }

    fn reserved_names(&mut self) -> Result<Vec<Node<String>>> {
        let mut names = Vec::new();

        loop {
            let str_node = self.string()?;

            names.push(str_node);

            if let Some(TokenKind::Comma) = self.peek_kind() {
                self.advance().unwrap();
            } else {
                break;
            }
        }

        Ok(names)
    }
}
