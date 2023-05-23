use super::{Node, TypeName};

#[derive(Debug, Clone)]
pub struct OptionNode {
    pub name: Node<OptionName>,
    pub value: Node<OptionValue>,
}

pub type OptionName = Vec<Node<OptionNamePart>>;

#[derive(Debug, Clone)]
pub enum OptionValue {
    StringLiteral(String),
    UintLiteral(String),
    IntLiteral(String),
    FloatLiteral(String),
    Identifier(String),
    MessageLiteral(String),
}

#[derive(Debug, Clone)]
pub enum OptionNamePart {
    SimpleName(String),
    ExtensionName(TypeName),
}
