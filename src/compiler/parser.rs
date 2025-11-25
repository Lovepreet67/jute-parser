use crate::compiler::{
    ast::{Class, Doc, Field, Module, Type},
    lexer::Lexer,
    state_machine::StateMachine,
    token::Token,
};
use std::error::Error;

pub struct Parser {
    name: String,
    sm: StateMachine,
    peek_count: usize,
    namespace: String,
    tokens: [Option<Token>; 2],
}

impl Parser {
    pub fn new(name: String, input: String) -> Self {
        let mut sm = StateMachine::new(Lexer::new(input));
        sm.start();

        println!("tokens : {:?}", sm.tokens);
        Parser {
            name,
            sm,
            peek_count: 0,
            namespace: "".to_string(),
            tokens: [None, None],
        }
    }
    // same as next token from the state machine it just skip the comment tokens
    fn next_token(&mut self) -> Option<Token> {
        while let Some(x) = self.sm.next_token() {
            match x {
                Token::SingleLineComment(_) | Token::MultiLineComment(_) => {}
                not_comment => return Some(not_comment),
            }
        }
        None
    }
    fn peek(&mut self) -> &Option<Token> {
        if self.peek_count > 0 {
            return &self.tokens[self.peek_count - 1];
        }
        self.peek_count = 1;
        self.tokens[0] = self.next_token();
        &self.tokens[0]
    }
    fn next(&mut self) -> Option<Token> {
        if self.peek_count > 0 {
            self.peek_count -= 1;
        } else {
            self.tokens[0] = self.next_token();
        }
        println!("Consuming {:?}", self.tokens[self.peek_count]);
        self.tokens[self.peek_count].take()
    }
    // helper function for expect

    fn expect(&mut self, token: Token) -> Result<(), Box<dyn Error>> {
        match self.next() {
            Some(x) if x == token => Ok(()),
            x => Err(format!("Error happend expexted {:?}, found {:?}", token, x).into()),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        match self.next() {
            Some(Token::Identifier(x)) => Ok(x),
            x => Err(format!("Error happend expexted identifier, found {:?}", x).into()),
        }
    }

    fn parse_include(&mut self) -> Result<String, Box<dyn Error>> {
        self.expect(Token::Include)?;
        self.expect_identifier()
    }
    fn parse_type(&mut self) -> Result<Type, Box<dyn Error>> {
        match self.peek() {
            Some(Token::Identifier(_)) => {
                // this means this type is referance to some other record
                self.parse_class_ref()
            }
            Some(Token::Vector) => self.parse_vector(),
            Some(Token::Map) => self.parse_map(),
            Some(Token::EOF) => Err("Unexpected end of file".into()),
            Some(x) => {
                let tor = x.to_premitive_type();
                self.next();
                tor
            }
            None => Err("Token not Found".into()),
        }
    }
    fn parse_vector(&mut self) -> Result<Type, Box<dyn Error>> {
        self.expect(Token::Vector)?;
        self.expect(Token::LAngleBrace)?;
        let t1 = self.parse_type()?;
        self.expect(Token::RAngleBrace)?;
        Ok(Type::Vector(Box::new(t1)))
    }
    fn parse_map(&mut self) -> Result<Type, Box<dyn Error>> {
        self.expect(Token::Map)?;
        self.expect(Token::LAngleBrace)?;
        let t1 = self.parse_type()?;
        self.expect(Token::Comma)?;
        let t2 = self.parse_type()?;
        self.expect(Token::RAngleBrace)?;
        Ok(Type::Map(Box::new(t1), Box::new(t2)))
    }
    fn parse_class_ref(&mut self) -> Result<Type, Box<dyn Error>> {
        let ident_val = self.expect_identifier()?;
        let last_index = ident_val.rfind('.').unwrap_or(0);
        let (mut namespace, classname) = if last_index > 0 {
            (&ident_val[..last_index], &ident_val[last_index + 1..])
        } else {
            ("", &ident_val[..])
        };
        if self.namespace == namespace {
            namespace = "";
        }
        Ok(Type::Class {
            name: classname.to_string(),
            namespace: namespace.to_string(),
        })
    }
    fn parse_field(&mut self) -> Result<Field, Box<dyn Error>> {
        let field_type = self.parse_type()?;
        let field_name = self.expect_identifier()?;
        self.expect(Token::Semicolon)?;
        Ok(Field {
            name: field_name,
            _type: field_type,
        })
    }
    fn parse_class(&mut self) -> Result<Class, Box<dyn Error>> {
        self.expect(Token::Class)?;
        let class_name = self.expect_identifier()?;
        self.expect(Token::LCurrelyBrace)?;
        let mut class = Class {
            name: class_name,
            fields: Vec::new(),
        };
        loop {
            match self.peek() {
                Some(Token::RCurrelyBrace) => {
                    self.next();
                    break;
                }
                _ => {
                    let field = self.parse_field()?;
                    class.fields.push(field);
                }
            }
        }
        Ok(class)
    }

    fn parse_module(&mut self) -> Result<Module, Box<dyn Error>> {
        self.expect(Token::Module)?;
        let module_name = self.expect_identifier()?;
        self.namespace = module_name.clone();
        let mut md = Module {
            name: module_name,
            records: Vec::new(),
        };
        self.expect(Token::LCurrelyBrace)?;
        loop {
            match self.peek() {
                Some(Token::Class) => {
                    let class = self.parse_class()?;
                    md.records.push(class);
                }
                Some(Token::RCurrelyBrace) => {
                    self.next();
                    break;
                }
                x => {
                    return Err(format!("Unexpected token {:?} found", x).into());
                }
            }
        }
        Ok(md)
    }

    pub fn parser(&mut self) -> Result<Doc, Box<dyn Error>> {
        let mut tor_doc = Doc::default();
        loop {
            match self.peek() {
                Some(Token::EOF) => {
                    break;
                }
                Some(Token::Include) => {
                    if let Ok(val) = self.parse_include() {
                        tor_doc.includes.push(val);
                    }
                }
                Some(Token::Module) => {
                    if let Ok(val) = self.parse_module() {
                        tor_doc.modules.push(val);
                    }
                }
                x => panic!("Tokens Ended other than EOF token {:?}", x),
            }
        }
        Ok(tor_doc)
    }
}
