use std::str::Chars;

pub struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
    current_pos: usize,
    current_line: usize,
    current_line_char: usize,
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            initial_len: input.len(),
            chars: input.chars(),
            current_pos: 0,
            current_line: 0,
            current_line_char: 0,
        }
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn reset_len_consumed(&mut self) {
        self.initial_len = self.chars.as_str().len()
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.current_pos += 1;
        self.current_line_char += 1;

        Some(c)
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn increment_line(&mut self) {
        self.current_line += 1;
        self.current_line_char = 0;
    }

    pub fn current_pos(&self) -> usize {
        self.current_pos
    }

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    pub fn current_line_char(&self) -> usize {
        self.current_line_char
    }
}
