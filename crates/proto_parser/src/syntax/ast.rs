use self::{
    message::{FieldDeclaration, MessageNode},
    option::OptionNode,
    service::ServiceNode,
};

mod message;
mod option;
mod service;

#[derive(Debug)]
pub enum RootNodeType {
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
    pub package_name: Vec<String>,
    pub modifier: Option<ImportModifier>,
}

#[derive(Debug)]
pub enum TypeName {
    Scalar(ScalarType),
    Message(String),
    Enum(String),
}

#[derive(Debug)]
pub enum Reserved {
    TagRanges(Vec<TagRange>),
    Names(Vec<String>),
}

#[derive(Debug)]
pub struct TagRange {
    start: u32,
    end: Option<TagEnd>,
}

#[derive(Debug)]
pub enum TagEnd {
    Tag(u32),
    Max,
}

#[derive(Debug)]
pub struct EnumNode {
    pub name: String,
    pub elements: Vec<EnumElement>,
}

#[derive(Debug)]
pub enum EnumElement {
    EnumValue {
        name: String,
        number: u32,
        options: Vec<OptionNode>,
    },
    EnumReserved(Reserved),
    EnumOption,
}

#[derive(Debug)]
pub struct ExtensionNode {
    pub extendee: String,
    pub elements: Vec<ExtensionElement>,
}

#[derive(Debug)]
pub enum ExtensionElement {
    Field(FieldDeclaration),
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
    pub nodes: Vec<RootNodeType>,
    pub file_name: String,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub start: usize,
    pub end: usize,
}
