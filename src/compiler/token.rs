use crate::{compiler::ast::Type, errors::JuteError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token {
    Error(String),
    SingleLineComment(String),
    MultiLineComment(String),
    LCurrelyBrace,
    RCurrelyBrace,
    LAngleBrace,
    RAngleBrace,
    Semicolon,
    Comma,
    QuotedString(String),
    Identifier(String),
    Keyword,
    Include,
    Module,
    Class,
    Byte,
    Bool,
    Int,
    LongInt,
    Float,
    Double,
    RString,
    Buffer,
    Vector,
    Map,
    EOF,
}

impl Token {
    pub fn from_str(segment: &str) -> Option<Token> {
        match segment {
            "include" => Some(Token::Include),
            "module" => Some(Token::Module),
            "class" => Some(Token::Class),
            "byte" => Some(Token::Byte),
            "int" => Some(Token::Int),
            "long" => Some(Token::LongInt),
            "float" => Some(Token::Float),
            "double" => Some(Token::Double),
            "ustring" | "string" => Some(Token::RString),
            "buffer" => Some(Token::Buffer),
            "vector" => Some(Token::Vector),
            "map" => Some(Token::Map),
            "boolean" => Some(Token::Bool),
            ";" => Some(Token::Semicolon),
            "," => Some(Token::Comma),
            "<" => Some(Token::LAngleBrace),
            ">" => Some(Token::RAngleBrace),
            "{" => Some(Token::LCurrelyBrace),
            "}" => Some(Token::RCurrelyBrace),
            _ => None,
        }
    }
    pub fn to_premitive_type(&self) -> Result<Type, JuteError> {
        match self {
            Token::Int => Ok(Type::Int),
            Token::LongInt => Ok(Type::Long),
            Token::Float => Ok(Type::Float),
            Token::Double => Ok(Type::Double),
            Token::Byte => Ok(Type::Byte),
            Token::Buffer => Ok(Type::Buffer),
            Token::Bool => Ok(Type::Boolean),
            Token::RString => Ok(Type::UString),
            x => Err(JuteError::InvalidConversionToPremitive { token: x.clone() }),
        }
    }
}
