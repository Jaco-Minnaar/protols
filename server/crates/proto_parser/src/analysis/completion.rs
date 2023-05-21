use crate::{syntax::Root, RootNode};

pub fn get_suggestions<'a>(roots: impl Iterator<Item = &'a Root>) -> Vec<String> {
    roots
        .flat_map(|root| {
            root.nodes
                .iter()
                .filter(|root_node| {
                    matches!(
                        root_node,
                        RootNode::MessageDeclaration(_) | RootNode::EnumDeclaration(_)
                    )
                })
                .map(|root_node| match root_node {
                    RootNode::MessageDeclaration(message_declaration) => {
                        message_declaration.value.name.clone()
                    }
                    RootNode::EnumDeclaration(enum_declaration) => {
                        enum_declaration.value.name.clone()
                    }
                    _ => unreachable!(),
                })
        })
        .collect()
}
