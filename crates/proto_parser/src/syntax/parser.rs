use std::{iter::Peekable, str::FromStr};

use crate::syntax::ast::{
    message::MessageNode, option::OptionNamePart, EnumNode, ExtensionNode, TypeName,
};

use super::{
    ast::{
        message::MessageElement,
        option::{OptionName, OptionNode, OptionValue},
        service::{MethodNode, ServiceNode},
        EnumElement, ExtensionElement, ImportModifier, ImportNode, Node, PackageNode, Root,
        RootNode, SyntaxNode, SyntaxType,
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
            Some(TokenKind::SyntaxKw) => self.syntax_node(),
            Some(TokenKind::PackageKw) => self.package_node(),
            Some(TokenKind::ImportKw) => self.import_node(),
            Some(TokenKind::OptionKw) => self.option_node(),
            Some(TokenKind::MessageKw) => self.message_node(),
            Some(TokenKind::EnumKw) => self.enum_node(),
            Some(TokenKind::ServiceKw) => self.service_node(),
            Some(TokenKind::ExtendKw) => self.extend_node(),
            _ => {
                let token = self.advance().unwrap();
                let err = ParseError::new(
                    format!("Unexpected token: {:?}", token.kind),
                    token.position,
                );
                Err(err)
            }
        };

        result
    }

    fn syntax_node(&mut self) -> Result<RootNode> {
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
        Ok(RootNode::SyntaxDeclaration(syntax_node))
    }

    fn package_node(&mut self) -> Result<RootNode> {
        let start = self.advance().unwrap().position;

        let name = self.qualified_identifier()?;

        let end_token = self.expect(TokenKind::SemiColon)?;

        self.advance().unwrap();

        let package_node = Node::new(
            PackageNode { package_name: name },
            start,
            end_token.position,
        );
        Ok(RootNode::PackageDeclaration(package_node))
    }

    fn import_node(&mut self) -> Result<RootNode> {
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

        Ok(RootNode::ImportDeclaration(import_node))
    }

    fn option_node(&mut self) -> Result<RootNode> {
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

        Ok(RootNode::OptionDeclaration(option_node))
    }

    fn message_node(&mut self) -> Result<RootNode> {
        let start = self.advance().unwrap().position;

        let identifier = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LBrace)?;

        let elements = self.message_elements()?;

        let end_token = self.expect(TokenKind::RBrace)?;

        let message_node = Node::new(
            MessageNode {
                name: identifier.value,
                elements,
            },
            start,
            end_token.position,
        );

        Ok(RootNode::MessageDeclaration(message_node))
    }

    fn enum_node(&mut self) -> Result<RootNode> {
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

        Ok(RootNode::EnumDeclaration(enum_node))
    }

    fn service_node(&mut self) -> Result<RootNode> {
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

        Ok(RootNode::ServiceDeclaration(service_node))
    }

    fn extend_node(&mut self) -> Result<RootNode> {
        let start = self.advance().unwrap().position;

        let identifier_token = self.qualified_identifier()?;

        self.expect(TokenKind::LBrace)?;

        let elements = self.extension_elements()?;

        let end_token = self.expect(TokenKind::RBrace)?;

        let extend_node = Node::new(
            ExtensionNode {
                extendee: identifier_token,
                elements,
            },
            start,
            end_token.position,
        );

        Ok(RootNode::ExtensionDeclaration(extend_node))
    }

    fn string(&mut self) -> Result<Token> {
        if !matches!(self.peek_kind(), Some(TokenKind::StringKw)) {
            let err = ParseError::new(
                format!("Expected string",),
                self.tokens.peek().unwrap().position,
            );
            return Err(err);
        }

        let mut token = self.advance().unwrap();

        let mut string = String::from_str(token.value.as_str()).unwrap();
        while let Some(TokenKind::StringKw) = self.peek_kind() {
            token = self.advance().unwrap();
            string.push_str(token.value.as_str());
        }

        let string = format!("\"{}\"", string.replace("\"", ""));

        let token = Token {
            kind: TokenKind::StringKw,
            value: string,
            position: token.position,
        };

        Ok(token)
    }

    fn qualified_identifier(&mut self) -> Result<Vec<String>> {
        if !matches!(self.peek_kind(), Some(TokenKind::Identifier)) {
            return Err(ParseError::new(
                format!("Expected identifier after '.'",),
                self.tokens.peek().unwrap().position,
            ));
        }

        let identifier = self.advance().unwrap();

        let mut identifiers = vec![identifier.value];

        while let Some(TokenKind::Dot) = self.peek_kind() {
            self.advance().unwrap();
            if !matches!(self.peek_kind(), Some(TokenKind::Identifier)) {
                let err = ParseError::new(
                    format!("Expected identifier after '.'",),
                    self.tokens.peek().unwrap().position,
                );
                return Err(err);
            }
            let identifier = self.advance().unwrap();

            identifiers.push(identifier.value);
        }

        Ok(identifiers)
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
                    let start = self.advance().unwrap().position;
                    let identifier = self.qualified_identifier()?;
                    let end = self.expect(TokenKind::RParen)?.position;

                    let part = Node::new(
                        OptionNamePart::ExtensionName(TypeName::from(identifier)),
                        start,
                        end,
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

    fn option_value(&self) -> Result<Node<OptionValue>> {
        todo!()
    }

    fn message_elements(&self) -> Result<Vec<Node<MessageElement>>> {
        todo!()
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
}
