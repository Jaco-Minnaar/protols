use crate::{message::Message, Node, Position, Root, RootDeclaration};

pub mod completion;

#[derive(Debug)]

pub struct ProtoMessage<'a> {
    pub node: &'a Node<RootDeclaration>,
    pub file: String,
}

pub fn get_messages<'a>(
    files: impl Iterator<Item = &'a Root> + 'a,
) -> impl Iterator<Item = ProtoMessage<'a>> + 'a {
    files.flat_map(|file| {
        file.nodes.iter().filter_map(|root_node| {
            if let RootDeclaration::Message(node) = &root_node.value {
                Some(ProtoMessage {
                    node: root_node,
                    file: file.file_name.clone(),
                })
            } else {
                None
            }
        })
    })
}

pub fn find_node<'a>(root: &'a Root, position: Position) -> Option<&'a str> {
    let start_idx = 0;
    let end_idx = root.nodes.len() - 1;

    while start_idx <= end_idx {
        let mid_idx = (start_idx + end_idx) / 2;
        let mid_node = &root.nodes[mid_idx];

        if mid_node.start.line <= position.line && mid_node.end.line >= position.line {
            return Some(&mid_node.value.name);
        }

        if mid_node.start.line > position.line {
            end_idx = mid_idx - 1;
        } else {
            start_idx = mid_idx + 1;
        }
    }

    None
}

fn find_node_name<'a>(decl: &'a RootDeclaration, position: Position) -> Option<&'a str> {
    match decl {
        RootDeclaration::Message(message) => message.elements.iter().find_map(|element| {
            if element.start.line <= position.line && element.end.line >= position.line {
                Some(&element.value.name)
            } else {
                None
            }
        }),
        RootDeclaration::Service(service) => service.elements.iter().find_map(|element| {
            if element.start.line <= position.line && element.end.line >= position.line {
                Some(&element.name)
            } else {
                None
            }
        }),
        _ => None,
    }
}
