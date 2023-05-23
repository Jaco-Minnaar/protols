use crate::Position;

use self::{
    message::{FieldDeclaration, Message},
    option::OptionNode,
    service::ServiceNode,
};

use super::lexer::{Keyword, TokenKind};

pub mod message;
pub mod option;
pub mod service;

#[derive(Debug)]
pub enum RootNode {
    SyntaxDeclaration(Node<SyntaxNode>),
    PackageDeclaration(Node<PackageNode>),
    ImportDeclaration(Node<ImportNode>),
    MessageDeclaration(Node<Message>),
    ServiceDeclaration(Node<ServiceNode>),
    OptionDeclaration(Node<OptionNode>),
    EnumDeclaration(Node<EnumNode>),
    ExtensionDeclaration(Node<ExtensionNode>),
    Empty,
}

#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub proto_type: SyntaxType,
}

#[derive(Debug, Clone)]
pub struct PackageNode {
    pub package_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub package_name: String,
    pub modifier: Option<ImportModifier>,
}

#[derive(Debug, Clone)]
pub struct TypeName {
    pub absolute: bool,
    pub parts: Vec<String>,
}

impl From<Vec<String>> for TypeName {
    fn from(value: Vec<String>) -> Self {
        Self {
            parts: value,
            absolute: false,
        }
    }
}

impl From<String> for TypeName {
    fn from(value: String) -> Self {
        let absolute = value.starts_with('.');
        let parts = value.split('.').map(|s| s.to_string()).collect();

        Self { parts, absolute }
    }
}

#[derive(Debug, Clone)]
pub enum Reserved {
    TagRanges(Vec<Node<TagRange>>),
    Names(Vec<Node<String>>),
}

#[derive(Debug, Clone)]
pub struct TagRange {
    pub start: Node<u32>,
    pub end: Option<Node<TagEnd>>,
}

#[derive(Debug, Clone)]
pub enum TagEnd {
    Tag(u32),
    Max,
}

#[derive(Debug, Clone)]
pub struct EnumNode {
    pub name: String,
    pub elements: Vec<Node<EnumElement>>,
}

#[derive(Debug, Clone)]
pub enum EnumElement {
    EnumValue {
        name: String,
        number: i32,
        options: Vec<Node<OptionNode>>,
    },
    EnumReserved(Reserved),
    EnumOption(OptionNode),
    Empty,
}

#[derive(Debug, Clone)]
pub struct ExtensionNode {
    pub extendee: Vec<String>,
    pub elements: Vec<Node<ExtensionElement>>,
}

#[derive(Debug, Clone)]
pub enum ExtensionElement {
    Field(FieldDeclaration),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MapKeyType {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
}

impl TryFrom<TokenKind> for MapKeyType {
    type Error = String;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Keyword(Keyword::Int32) => Ok(MapKeyType::Int32),
            TokenKind::Keyword(Keyword::Int64) => Ok(MapKeyType::Int64),
            TokenKind::Keyword(Keyword::Uint32) => Ok(MapKeyType::Uint32),
            TokenKind::Keyword(Keyword::Uint64) => Ok(MapKeyType::Uint64),
            TokenKind::Keyword(Keyword::Sint32) => Ok(MapKeyType::Sint32),
            TokenKind::Keyword(Keyword::Sint64) => Ok(MapKeyType::Sint64),
            TokenKind::Keyword(Keyword::Fixed32) => Ok(MapKeyType::Fixed32),
            TokenKind::Keyword(Keyword::Fixed64) => Ok(MapKeyType::Fixed64),
            TokenKind::Keyword(Keyword::SFixed32) => Ok(MapKeyType::Sfixed32),
            TokenKind::Keyword(Keyword::SFixed64) => Ok(MapKeyType::Sfixed64),
            TokenKind::Keyword(Keyword::Bool) => Ok(MapKeyType::Bool),
            TokenKind::Keyword(Keyword::String) => Ok(MapKeyType::String),
            _ => Err(format!("Invalid map key type: {:?}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScalarType {
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,
}

impl TryFrom<TokenKind> for ScalarType {
    type Error = String;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        let val = match value {
            TokenKind::Keyword(Keyword::Double) => ScalarType::Double,
            TokenKind::Keyword(Keyword::Float) => ScalarType::Float,
            TokenKind::Keyword(Keyword::Int32) => ScalarType::Int32,
            TokenKind::Keyword(Keyword::Int64) => ScalarType::Int64,
            TokenKind::Keyword(Keyword::Uint32) => ScalarType::Uint32,
            TokenKind::Keyword(Keyword::Uint64) => ScalarType::Uint64,
            TokenKind::Keyword(Keyword::Sint32) => ScalarType::Sint32,
            TokenKind::Keyword(Keyword::Sint64) => ScalarType::Sint64,
            TokenKind::Keyword(Keyword::Fixed32) => ScalarType::Fixed32,
            TokenKind::Keyword(Keyword::Fixed64) => ScalarType::Fixed64,
            TokenKind::Keyword(Keyword::SFixed32) => ScalarType::Sfixed32,
            TokenKind::Keyword(Keyword::SFixed64) => ScalarType::Sfixed64,
            TokenKind::Keyword(Keyword::Bool) => ScalarType::Bool,
            TokenKind::Keyword(Keyword::String) => ScalarType::String,
            TokenKind::Keyword(Keyword::Bytes) => ScalarType::Bytes,
            _ => return Err(format!("Invalid scalar type: {:?}", value)),
        };

        Ok(val)
    }
}
impl TryFrom<&TokenKind> for ScalarType {
    type Error = String;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        let val = match value {
            TokenKind::Keyword(Keyword::Double) => ScalarType::Double,
            TokenKind::Keyword(Keyword::Float) => ScalarType::Float,
            TokenKind::Keyword(Keyword::Int32) => ScalarType::Int32,
            TokenKind::Keyword(Keyword::Int64) => ScalarType::Int64,
            TokenKind::Keyword(Keyword::Uint32) => ScalarType::Uint32,
            TokenKind::Keyword(Keyword::Uint64) => ScalarType::Uint64,
            TokenKind::Keyword(Keyword::Sint32) => ScalarType::Sint32,
            TokenKind::Keyword(Keyword::Sint64) => ScalarType::Sint64,
            TokenKind::Keyword(Keyword::Fixed32) => ScalarType::Fixed32,
            TokenKind::Keyword(Keyword::Fixed64) => ScalarType::Fixed64,
            TokenKind::Keyword(Keyword::SFixed32) => ScalarType::Sfixed32,
            TokenKind::Keyword(Keyword::SFixed64) => ScalarType::Sfixed64,
            TokenKind::Keyword(Keyword::Bool) => ScalarType::Bool,
            TokenKind::Keyword(Keyword::String) => ScalarType::String,
            TokenKind::Keyword(Keyword::Bytes) => ScalarType::Bytes,
            _ => return Err(format!("Invalid scalar type: {:?}", value)),
        };

        Ok(val)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxType {
    Proto2,
    Proto3,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportModifier {
    Weak,
    Public,
}

#[derive(Debug)]
pub struct Root {
    pub nodes: Vec<RootNode>,
    pub file_name: String,
}

impl Root {
    pub fn new(file_name: String) -> Self {
        Self {
            nodes: Vec::new(),
            file_name,
        }
    }

    pub fn add_node(&mut self, node: RootNode) {
        self.nodes.push(node);
    }
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: Clone,
{
    pub value: T,
    pub start: Position,
    pub end: Position,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(value: T, start: Position, end: Position) -> Self {
        Self { value, start, end }
    }
}
