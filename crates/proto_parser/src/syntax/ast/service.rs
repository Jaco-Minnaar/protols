use super::{option::OptionNode, Node, TypeName};

#[derive(Debug)]
pub struct ServiceNode {
    pub name: String,
    pub methods: Vec<Node<MethodNode>>,
}

#[derive(Debug)]
pub struct MethodNode {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub elements: Vec<MethodElement>,
}

#[derive(Debug)]
pub enum MethodElement {
    Option(OptionNode),
    Empty,
}

#[derive(Debug)]
pub struct MessageType {
    pub stream: bool,
    pub type_name: TypeName,
}
