use super::{Node, TypeName};

#[derive(Debug)]
pub struct OptionNode {
    pub name: Node<OptionName>,
    pub value: Node<OptionValue>,
}

pub type OptionName = Vec<Node<OptionNamePart>>;

#[derive(Debug)]
pub enum OptionValue {
    StringLiteral(String),
    UintLiteral(String),
    IntLiteral(String),
    FloatLiteral(String),
    Identifier(String),
    MessageLiteral(String),
}

#[derive(Debug)]
pub enum OptionNamePart {
    SimpleName(String),
    ExtensionName(TypeName),
}
