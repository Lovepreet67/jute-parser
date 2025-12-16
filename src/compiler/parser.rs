use crate::{
    compiler::{
        ast::{Class, Doc, Field, Module, Type},
        lexer::Lexer,
        state_machine::StateMachine,
        token::Token,
    },
    errors::JuteError,
};
use std::path::{Path, PathBuf};

pub struct Parser {
    src_file_location: PathBuf,
    sm: StateMachine,
    namespace: String,
    next_token: Option<Token>,
}

impl Parser {
    pub fn new(src_file_location: PathBuf, input: String) -> Result<Self, JuteError> {
        let mut sm = StateMachine::new(Lexer::new(input));
        sm.start()?;
        Ok(Parser {
            src_file_location,
            sm,
            namespace: "".to_string(),
            next_token: None,
        })
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
        if self.next_token.is_some() {
            return &self.next_token;
        }
        self.next_token = self.next_token();
        &self.next_token
    }
    fn next(&mut self) -> Option<Token> {
        if self.next_token.is_none() {
            self.next_token = self.next_token();
        }
        self.next_token.take()
    }
    // helper function for expect

    fn expect(&mut self, token: Token) -> Result<(), JuteError> {
        match self.next() {
            Some(x) if x == token => Ok(()),
            x => Err(JuteError::UnexpectedToken {
                token: Some(format!("{:?}", x)),
                message: format!("Expecting token {:?}", token),
            }),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, JuteError> {
        match self.next() {
            Some(Token::Identifier(x)) => Ok(x),
            Some(Token::QuotedString(x)) => Ok(x),
            x => {
                return Err(JuteError::UnexpectedToken {
                    token: Some(format!("{:?}", x)),
                    message: format!("Expecting an identifier"),
                });
            }
        }
    }

    fn parse_include(&mut self) -> Result<PathBuf, JuteError> {
        self.expect(Token::Include)?;
        let relative_include_path = self.expect_identifier()?;
        let base_dir = self
            .src_file_location
            .parent()
            .unwrap_or_else(|| Path::new("."));
        Ok(base_dir.join(relative_include_path))
    }
    fn parse_type(&mut self) -> Result<Type, JuteError> {
        match self.peek() {
            Some(Token::Identifier(_)) => {
                // this means this type is referance to some other record
                self.parse_class_ref()
            }
            Some(Token::Vector) => self.parse_vector(),
            Some(Token::Map) => self.parse_map(),
            Some(Token::EOF) => Err(JuteError::UnexpectedEndOfFile),
            Some(x) => {
                let tor = x.to_premitive_type();
                self.next();
                tor
            }
            None => Err(JuteError::UnexpectedEndOfTokenStream),
        }
    }
    fn parse_vector(&mut self) -> Result<Type, JuteError> {
        self.expect(Token::Vector)?;
        self.expect(Token::LAngleBrace)?;
        let t1 = self.parse_type()?;
        self.expect(Token::RAngleBrace)?;
        Ok(Type::Vector(Box::new(t1)))
    }
    fn parse_map(&mut self) -> Result<Type, JuteError> {
        self.expect(Token::Map)?;
        self.expect(Token::LAngleBrace)?;
        let t1 = self.parse_type()?;
        self.expect(Token::Comma)?;
        let t2 = self.parse_type()?;
        self.expect(Token::RAngleBrace)?;
        Ok(Type::Map(Box::new(t1), Box::new(t2)))
    }
    fn parse_class_ref(&mut self) -> Result<Type, JuteError> {
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
    fn parse_field(&mut self) -> Result<Field, JuteError> {
        let field_type = self.parse_type()?;
        let field_name = self.expect_identifier()?;
        self.expect(Token::Semicolon)?;
        Ok(Field {
            name: field_name,
            _type: field_type,
        })
    }
    fn parse_class(&mut self) -> Result<Class, JuteError> {
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

    fn parse_module(&mut self) -> Result<Module, JuteError> {
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
                    return Err(JuteError::UnexpectedToken {
                        token: Some(format!("{:?}", x)),
                        message: "Expected module end or new class start".to_owned(),
                    });
                }
            }
        }
        Ok(md)
    }

    pub fn parse(&mut self) -> Result<Doc, JuteError> {
        let mut tor_doc = Doc {
            src: self.src_file_location.canonicalize().map_err(|e| {
                return JuteError::PathCanonicalizeError {
                    message: e.to_string(),
                };
            })?,
            ..Default::default()
        };
        loop {
            match self.peek() {
                Some(Token::EOF) => {
                    break;
                }
                Some(Token::Include) => {
                    if let Ok(val) = self.parse_include() {
                        tor_doc.includes.push(val.canonicalize().map_err(|e| {
                            return JuteError::PathCanonicalizeError {
                                message: e.to_string(),
                            };
                        })?);
                    }
                    self.expect(Token::Semicolon)?
                }
                Some(Token::Module) => {
                    if let Ok(val) = self.parse_module() {
                        tor_doc.modules.push(val);
                    }
                }
                Some(x) => {
                    return Err(JuteError::UnexpectedToken {
                        token: Some(format!("{:?}", x)),
                        message: "unexpected token found".to_owned(),
                    });
                }
                None => return Err(JuteError::UnexpectedEndOfTokenStream),
            }
        }

        Ok(tor_doc)
    }
}
