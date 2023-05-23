use crate::{message::Message, Node, Root, RootNode};

pub mod completion;

#[derive(Debug)]

pub struct ProtoMessage<'a> {
    pub node: &'a Node<Message>,
    pub file: String,
}

pub fn get_messages<'a>(
    files: impl Iterator<Item = &'a Root> + 'a,
) -> impl Iterator<Item = ProtoMessage<'a>> + 'a {
    files.flat_map(|file| {
        file.nodes.iter().filter_map(|root_node| {
            if let RootNode::MessageDeclaration(node) = root_node {
                Some(ProtoMessage {
                    node,
                    file: file.file_name.clone(),
                })
            } else {
                None
            }
        })
    })
}
