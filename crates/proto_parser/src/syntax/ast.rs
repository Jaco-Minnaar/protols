use self::{
    message::{FieldDeclaration, MessageNode},
    option::OptionNode,
    service::ServiceNode,
};

use super::lexer::TokenKind;

pub mod message;
pub mod option;
pub mod service;

#[derive(Debug)]
pub enum RootNode {
    SyntaxDeclaration(Node<SyntaxNode>),
    PackageDeclaration(Node<PackageNode>),
    ImportDeclaration(Node<ImportNode>),
    MessageDeclaration(Node<MessageNode>),
    ServiceDeclaration(Node<ServiceNode>),
    OptionDeclaration(Node<OptionNode>),
    EnumDeclaration(Node<EnumNode>),
    ExtensionDeclaration(Node<ExtensionNode>),
}

#[derive(Debug)]
pub struct SyntaxNode {
    pub proto_type: SyntaxType,
}

#[derive(Debug)]
pub struct PackageNode {
    pub package_name: Vec<String>,
}

#[derive(Debug)]
pub struct ImportNode {
    pub package_name: String,
    pub modifier: Option<ImportModifier>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Reserved {
    TagRanges(Vec<Node<TagRange>>),
    Names(Vec<Node<String>>),
}

#[derive(Debug)]
pub struct TagRange {
    pub start: Node<u32>,
    pub end: Option<Node<TagEnd>>,
}

#[derive(Debug)]
pub enum TagEnd {
    Tag(u32),
    Max,
}

#[derive(Debug)]
pub struct EnumNode {
    pub name: String,
    pub elements: Vec<Node<EnumElement>>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ExtensionNode {
    pub extendee: Vec<String>,
    pub elements: Vec<Node<ExtensionElement>>,
}

#[derive(Debug)]
pub enum ExtensionElement {
    Field(FieldDeclaration),
}

#[derive(Debug, PartialEq, Eq)]
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
            TokenKind::Int32Kw => Ok(MapKeyType::Int32),
            TokenKind::Int64Kw => Ok(MapKeyType::Int64),
            TokenKind::Uint32Kw => Ok(MapKeyType::Uint32),
            TokenKind::Uint64Kw => Ok(MapKeyType::Uint64),
            TokenKind::Sint32Kw => Ok(MapKeyType::Sint32),
            TokenKind::Sint64Kw => Ok(MapKeyType::Sint64),
            TokenKind::Fixed32Kw => Ok(MapKeyType::Fixed32),
            TokenKind::Fixed64Kw => Ok(MapKeyType::Fixed64),
            TokenKind::SFixed32Kw => Ok(MapKeyType::Sfixed32),
            TokenKind::SFixed64Kw => Ok(MapKeyType::Sfixed64),
            TokenKind::BoolKw => Ok(MapKeyType::Bool),
            TokenKind::StringKw => Ok(MapKeyType::String),
            _ => Err(format!("Invalid map key type: {:?}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
            TokenKind::DoubleKw => ScalarType::Double,
            TokenKind::FloatKw => ScalarType::Float,
            TokenKind::Int32Kw => ScalarType::Int32,
            TokenKind::Int64Kw => ScalarType::Int64,
            TokenKind::Uint32Kw => ScalarType::Uint32,
            TokenKind::Uint64Kw => ScalarType::Uint64,
            TokenKind::Sint32Kw => ScalarType::Sint32,
            TokenKind::Sint64Kw => ScalarType::Sint64,
            TokenKind::Fixed32Kw => ScalarType::Fixed32,
            TokenKind::Fixed64Kw => ScalarType::Fixed64,
            TokenKind::SFixed32Kw => ScalarType::Sfixed32,
            TokenKind::SFixed64Kw => ScalarType::Sfixed64,
            TokenKind::BoolKw => ScalarType::Bool,
            TokenKind::StringKw => ScalarType::String,
            TokenKind::BytesKw => ScalarType::Bytes,
            _ => return Err(format!("Invalid scalar type: {:?}", value)),
        };

        Ok(val)
    }
}
impl TryFrom<&TokenKind> for ScalarType {
    type Error = String;

    fn try_from(value: &TokenKind) -> Result<Self, Self::Error> {
        let val = match value {
            TokenKind::DoubleKw => ScalarType::Double,
            TokenKind::FloatKw => ScalarType::Float,
            TokenKind::Int32Kw => ScalarType::Int32,
            TokenKind::Int64Kw => ScalarType::Int64,
            TokenKind::Uint32Kw => ScalarType::Uint32,
            TokenKind::Uint64Kw => ScalarType::Uint64,
            TokenKind::Sint32Kw => ScalarType::Sint32,
            TokenKind::Sint64Kw => ScalarType::Sint64,
            TokenKind::Fixed32Kw => ScalarType::Fixed32,
            TokenKind::Fixed64Kw => ScalarType::Fixed64,
            TokenKind::SFixed32Kw => ScalarType::Sfixed32,
            TokenKind::SFixed64Kw => ScalarType::Sfixed64,
            TokenKind::BoolKw => ScalarType::Bool,
            TokenKind::StringKw => ScalarType::String,
            TokenKind::BytesKw => ScalarType::Bytes,
            _ => return Err(format!("Invalid scalar type: {:?}", value)),
        };

        Ok(val)
    }
}

#[derive(Debug, PartialEq)]
pub enum SyntaxType {
    Proto2,
    Proto3,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub start: usize,
    pub end: usize,
}

impl<T> Node<T> {
    pub fn new(value: T, start: usize, end: usize) -> Self {
        Self { value, start, end }
    }
}
