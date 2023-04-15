use super::{
    option::OptionNode, EnumElement, ExtensionElement, Node, Reserved, ScalarType, TypeName,
};

#[derive(Debug)]
pub struct MessageNode {
    pub name: String,
    pub elements: Vec<Node<MessageElement>>,
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub cardinality: FieldCardinality,
    pub type_name: FieldType,
    pub name: String,
    pub number: u32,
    pub options: Vec<OptionNode>,
}

#[derive(Debug)]
pub struct MapFieldDeclaration {
    pub key_type: ScalarType,
    pub value_type: FieldType,
    pub name: String,
    pub number: u32,
    pub options: Vec<OptionNode>,
}

#[derive(Debug)]
pub enum MessageElement {
    FieldDeclaration(FieldDeclaration),
    MapFieldDeclaration(MapFieldDeclaration),
    OneOfDeclaration {
        name: String,
        elements: Vec<OneofElement>,
    },
    OptionDeclaration(OptionNode),
    ReservedDeclaration(Reserved),
    MessageDeclaration(Box<MessageElement>),
    EnumDeclaration {
        name: String,
        elements: Vec<EnumElement>,
    },
    ExtensionDeclaration {
        extendee: String,
        elements: Vec<ExtensionElement>,
    },
    EmptyDeclaration,
}

#[derive(Debug)]
pub enum FieldCardinality {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug)]
pub enum FieldType {
    ScalarType(ScalarType),
    TypeName(TypeName),
}

#[derive(Debug)]
pub enum OneofElement {
    Option,
    OneofField {
        type_name: FieldType,
        name: String,
        number: u32,
        options: Vec<OptionNode>,
    },
}
