use crate::syntax::lexer::TokenKind;

use super::{
    option::OptionNode, EnumNode, ExtensionNode, MapKeyType, Node, Reserved, ScalarType, TypeName,
};

#[derive(Debug)]
pub struct MessageNode {
    pub name: String,
    pub elements: Vec<Node<MessageElement>>,
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub cardinality: Node<FieldCardinality>,
    pub type_name: Node<FieldType>,
    pub name: Node<String>,
    pub number: Node<u32>,
    pub options: Vec<Node<OptionNode>>,
}

#[derive(Debug)]
pub struct MapFieldDeclaration {
    pub key_type: Node<MapKeyType>,
    pub value_type: Node<FieldType>,
    pub name: Node<String>,
    pub number: Node<u32>,
    pub options: Vec<Node<OptionNode>>,
}

#[derive(Debug)]
pub enum MessageElement {
    FieldDeclaration(FieldDeclaration),
    MapFieldDeclaration(MapFieldDeclaration),
    OneOfDeclaration(OneofDeclaration),
    OptionDeclaration(OptionNode),
    ReservedDeclaration(Reserved),
    MessageDeclaration(Box<MessageNode>),
    EnumDeclaration(EnumNode),
    ExtensionDeclaration(ExtensionNode),
    EmptyDeclaration,
}

#[derive(Debug)]
pub struct OneofDeclaration {
    pub name: Node<String>,
    pub elements: Vec<Node<OneofElement>>,
}

#[derive(Debug)]
pub enum FieldCardinality {
    Required,
    Optional,
    Repeated,
}

impl TryFrom<TokenKind> for FieldCardinality {
    type Error = String;

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::RequiredKw => Ok(FieldCardinality::Required),
            TokenKind::OptionalKw => Ok(FieldCardinality::Optional),
            TokenKind::RepeatedKw => Ok(FieldCardinality::Repeated),
            _ => Err(format!("Invalid field cardinality: {:?}", value)),
        }
    }
}

#[derive(Debug)]
pub enum FieldType {
    ScalarType(ScalarType),
    TypeName(TypeName),
}

#[derive(Debug)]
pub enum OneofElement {
    Option(OptionNode),
    OneofField(OneofField),
}

#[derive(Debug)]
pub struct OneofField {
    pub type_name: Node<FieldType>,
    pub name: Node<String>,
    pub number: Node<u32>,
    pub options: Option<Vec<Node<OptionNode>>>,
}
