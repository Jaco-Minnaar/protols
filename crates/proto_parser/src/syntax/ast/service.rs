use super::{option::OptionNode, Node, TypeName};

#[derive(Debug)]
pub struct ServiceNode {
    pub name: String,
    pub elements: Vec<Node<ServiceElement>>,
}

#[derive(Debug)]
pub enum ServiceElement {
    Option(OptionNode),
    Method(MethodNode),
    Empty,
}

#[derive(Debug)]
pub struct MethodNode {
    pub name: String,
    pub input_type: Node<MessageType>,
    pub output_type: Node<MessageType>,
    pub elements: Vec<Node<MethodElement>>,
}

#[derive(Debug)]
pub enum MethodElement {
    Option(OptionNode),
    Empty,
}

#[derive(Debug)]
pub struct MessageType {
    pub stream: bool,
    pub type_name: Node<TypeName>,
}
