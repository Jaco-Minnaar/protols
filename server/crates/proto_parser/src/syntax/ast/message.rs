use crate::syntax::lexer::{Keyword, TokenKind};

use super::{
    option::OptionNode, EnumNode, ExtensionNode, MapKeyType, Node, Reserved, ScalarType, TypeName,
};

#[derive(Debug, Clone)]
pub struct Message {
    pub name: String,
    pub elements: Vec<Node<MessageElement>>,
}

#[derive(Debug, Clone)]
pub struct FieldDeclaration {
    pub cardinality: Node<FieldCardinality>,
    pub type_name: Node<FieldType>,
    pub name: Node<String>,
    pub number: Node<u32>,
    pub options: Vec<Node<OptionNode>>,
}

#[derive(Debug, Clone)]
pub struct MapFieldDeclaration {
    pub key_type: Node<MapKeyType>,
    pub value_type: Node<FieldType>,
    pub name: Node<String>,
    pub number: Node<u32>,
    pub options: Vec<Node<OptionNode>>,
}

#[derive(Debug, Clone)]
pub enum MessageElement {
    Field(FieldDeclaration),
    MapField(MapFieldDeclaration),
    OneOf(OneofDeclaration),
    Option(OptionNode),
    Reserved(Reserved),
    Message(Box<Message>),
    Enum(EnumNode),
    Extension(ExtensionNode),
    Empty,
}

#[derive(Debug, Clone)]
pub struct OneofDeclaration {
    pub name: Node<String>,
    pub elements: Vec<Node<OneofElement>>,
}

#[derive(Debug, Clone)]
pub enum FieldCardinality {
    Required,
    Optional,
    Repeated,
}

impl TryFrom<TokenKind> for FieldCardinality {
    type Error = String;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Keyword(Keyword::Required) => Ok(FieldCardinality::Required),
            TokenKind::Keyword(Keyword::Optional) => Ok(FieldCardinality::Optional),
            TokenKind::Keyword(Keyword::Repeated) => Ok(FieldCardinality::Repeated),
            _ => Err(format!("Invalid field cardinality: {:?}", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldType {
    ScalarType(ScalarType),
    TypeName(TypeName),
}

#[derive(Debug, Clone)]
pub enum OneofElement {
    Option(OptionNode),
    OneofField(OneofField),
}

#[derive(Debug, Clone)]
pub struct OneofField {
    pub type_name: Node<FieldType>,
    pub name: Node<String>,
    pub number: Node<u32>,
    pub options: Option<Vec<Node<OptionNode>>>,
}
