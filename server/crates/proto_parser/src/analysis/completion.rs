use crate::{syntax::Root, RootDeclaration};

pub fn get_suggestions<'a>(roots: impl Iterator<Item = &'a Root>) -> Vec<String> {
    roots
        .flat_map(|root| {
            root.nodes
                .iter()
                .filter(|root_node| {
                    matches!(
                        root_node.value,
                        RootDeclaration::Message(_) | RootDeclaration::Enum(_)
                    )
                })
                .map(|root_node| match root_node.value {
                    RootDeclaration::Message(message_declaration) => {
                        message_declaration.name.clone()
                    }
                    RootDeclaration::Enum(enum_declaration) => enum_declaration.name.clone(),
                    _ => unreachable!(),
                })
        })
        .collect()
}
