use std::collections::HashMap;

use crate::{completion::get_suggestions, tokenize, ParseResult, Parser};

#[derive(Debug)]
pub struct Source {
    trees: HashMap<String, ParseResult>,
    completions: Vec<String>,
}

impl Source {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
            completions: Vec::new(),
        }
    }

    pub fn parse(&mut self, name: &str, source: &str) {
        let tokens = tokenize(source);
        let tree = Parser::new(tokens).parse(name);

        self.trees.insert(name.to_string(), tree);
        self.completions = get_suggestions(self.trees.values().map(|result| &result.root));
    }

    pub fn completions(&self, _line: usize, _column: usize) -> Vec<String> {
        self.completions
            .iter()
            .map(|completion| completion.to_string())
            .collect()
    }
}
