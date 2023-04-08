use super::TypeName;

#[derive(Debug)]
pub struct OptionNode {
    pub name: OptionName,
    pub value: OptionValue,
}

#[derive(Debug)]
pub enum OptionName {
    SimpleName(String),
    ExtensionName(TypeName),
}

#[derive(Debug)]
pub enum OptionValue {
    StringLiteral(String),
    UintLiteral(String),
    IntLiteral(String),
    FloatLiteral(String),
    Identifier(String),
    MessageLiteral(String),
}
