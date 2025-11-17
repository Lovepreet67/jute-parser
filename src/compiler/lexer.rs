use crate::compiler::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    start: usize,
    pos: usize,
    width: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            start: 0,
            pos: 0,
            width: 0,
            tokens: Vec::new(),
        }
    }
    pub fn next(&mut self) -> Option<char> {
        if let Some(next_char) = self.input[self.pos..].chars().next() {
            self.width = next_char.len_utf8();
            self.pos += self.width;
            return Some(next_char);
        }
        self.width = 0;
        None
    }
    pub fn move_back(&mut self) {
        self.pos -= self.width
    }
    pub fn ignore(&mut self) {
        self.start = self.pos
    }
    pub fn emit_token(&mut self) -> String {
        let tor = &self.input[self.start..self.pos];
        self.start = self.pos;
        tor.to_string()
    }
    pub fn ignore_whitespace(&mut self) {
        while let Some(is_whitespace) = self.next().map(|c| c.is_whitespace())
            && is_whitespace
        {}
        self.move_back();
        self.ignore();
    }

    // now we will implement the state machine for this
}
