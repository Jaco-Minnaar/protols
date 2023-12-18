mod ast;
mod cursor;
mod lexer;
mod parser;

use std::ops::{Add, Sub};

pub use ast::*;
pub use lexer::tokenize;
pub use parser::{ParseError, ParseResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Sub<usize> for &Position {
    type Output = Position;

    fn sub(self, rhs: usize) -> Self::Output {
        Position {
            line: self.line,
            column: self.column - rhs,
        }
    }
}

impl Add<usize> for &Position {
    type Output = Position;

    fn add(self, rhs: usize) -> Self::Output {
        Position {
            line: self.line,
            column: self.column + rhs,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: Default::default(),
            column: Default::default(),
        }
    }
}
