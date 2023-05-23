use std::collections::HashMap;

use crate::{completion::get_suggestions, get_messages, tokenize, ParseResult, Parser, Position};

#[derive(Debug)]
pub struct Source {
    trees: HashMap<String, ParseResult>,
    completions: Vec<String>,
    messages: HashMap<String, Position>,
}

impl Source {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
            completions: Vec::new(),
            messages: HashMap::new(),
        }
    }

    pub fn parse(&mut self, name: &str, source: &str) {
        let tokens = tokenize(source);
        let tree = Parser::new(tokens).parse(name);

        self.trees.insert(name.to_string(), tree);
        self.completions = get_suggestions(self.trees.values().map(|result| &result.root));
        let messages = get_messages(self.trees.values().map(|result| &result.root));
        self.messages = messages
            .map(|message| (message.node.value.name.clone(), message.node.start))
            .collect();
    }

    pub fn completions(&self, _line: usize, _column: usize) -> Vec<String> {
        self.completions
            .iter()
            .map(|completion| completion.to_string())
            .collect()
    }

    pub fn goto_definition(&self, name: &str) -> Option<Position> {
        self.messages.get(name).copied()
    }
}
