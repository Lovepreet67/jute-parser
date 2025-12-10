use crate::compiler::{lexer::Lexer, token::Token};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct StateMachine {
    lex: Lexer,
    pub tokens: VecDeque<Token>,
}
impl StateMachine {
    pub fn new(lex: Lexer) -> Self {
        Self {
            lex,
            tokens: VecDeque::new(),
        }
    }
    pub fn start(&mut self) {
        loop {
            self.lex.ignore_whitespace();
            let current_char = self.lex.next();
            match current_char {
                Some('/') => {
                    let next_char = self.lex.next().expect("File Ended with unexpected token /");
                    match next_char {
                        '/' => {
                            self.process_single_line_comment();
                        }
                        '*' => {
                            self.process_multi_line_comment();
                        }
                        x => {
                            panic!("Unexpected token {x} found");
                        }
                    }
                }
                Some(token) if Token::from_str(&token.to_string()).is_some() => {
                    self.tokens
                        .push_back(Token::from_str(&token.to_string()).unwrap());
                }
                Some('"') => {
                    self.lex.ignore();
                    self.process_quoted_text();
                }
                Some(letter) if letter.is_ascii() => {
                    self.process_identifier();
                }
                None => {
                    self.tokens.push_back(Token::EOF);
                    return;
                }
                Some(x) => {
                    panic!("Unexpexted character {x} found")
                }
            }
        }
    }
    // we will keep the single line text as it is leading //
    fn process_single_line_comment(&mut self) {
        while let Some(x) = self.lex.next()
            && x != '\n'
        {}
        self.tokens
            .push_back(Token::SingleLineComment(self.lex.emit_token()));
    }

    // we will keep the single line text as it is leading //
    fn process_multi_line_comment(&mut self) {
        while let Some(x) = self.lex.next() {
            if x == '*' {
                match self.lex.next() {
                    Some('/') => {
                        // this will contain the '*' at the end
                        self.tokens
                            .push_back(Token::MultiLineComment(self.lex.emit_token()));
                        return;
                    }
                    _ => self.lex.move_back(),
                }
            }
        }
        panic!("Unterminated Multiline comment");
    }

    // qouted strings are stored without quotes
    fn process_quoted_text(&mut self) {
        while let Some(val) = self.lex.next() {
            match val {
                '\n' => {
                    panic!("Quoted Strings can't have new line character");
                }
                '"' => {
                    self.lex.move_back();
                    self.tokens
                        .push_back(Token::QuotedString(self.lex.emit_token()));
                    self.lex.next();
                    return;
                }
                _ => {}
            }
        }
        panic!("Unterminated quoted string");
    }
    fn process_identifier(&mut self) {
        while let Some(val) = self.lex.next()
            && (val.is_alphanumeric() || val == '.' || val == '_')
        {}
        // as we know file will not end with identifier at the end so loop will terminate because
        // char is not alpha numric so we will move back from the last char processed
        self.lex.move_back();
        let token_value = self.lex.emit_token();
        // if token is some kind of identifier
        if let Some(token) = Token::from_str(&token_value) {
            self.tokens.push_back(token);
        } else {
            self.tokens.push_back(Token::Identifier(token_value));
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;
    // TODO: update this test
    #[test]
    pub fn test_tokenizer() {
        // read file from the test directory
        let input = read_to_string("./test.jute").unwrap();
        let lexer = Lexer::new(input);
        let mut sm = StateMachine::new(lexer);
        sm.start();
        for token in sm.tokens {
            println!("{:?}", token);
        }
    }
}
