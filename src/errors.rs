use std::{fmt, path::PathBuf};

use crate::compiler::{ast::Type, token::Token};

#[derive(Debug)]
pub enum TypeValidationError {
    UnknownType,
    AmbigousType,
}

#[derive(Debug)]
pub enum JuteError {
    // used errors
    InvalidConversionToPremitive {
        token: Token,
    },
    UnexpectedToken {
        token: Option<Token>,
        message: String,
    },
    UnexpectedChar {
        c: char,
        message: String,
    },
    UnexpectedEndOfTokenStream,
    UnexpectedEndOfFile,
    PathCanonicalizeError {
        message: String,
    },
    UnTerminatedMultiLineComment,
    UnTerminatedQuotedString,
    QuotedStringWithNewLineCharacter,
    UnknownType {
        name: String,
        _type: Type,
        record: String,
        module: String,
        file: PathBuf,
    },
    AmbiguousType {
        name: String,
        _type: Type,
        record: String,
        module: String,
        file: PathBuf,
    },
    CircularDependency {
        cycle: String,
    },
    Io(std::io::Error),
    Fmt(std::fmt::Error),
    Utf8(std::string::FromUtf8Error),
    DuplicateModuleName {
        module_name: String,
        file_name: PathBuf,
    },
    DuplicateClassName {
        class_name: String,
        module_name: String,
        file_name: PathBuf,
    },
    DuplicateFieldName {
        field_name: String,
        class_name: String,
        module_name: String,
        file_name: PathBuf,
    },
    InvalidFieldType {
        field_type: String,
        field_name: String,
        class_name: String,
        module_name: String,
        file_name: PathBuf,
    },
}

impl From<std::io::Error> for JuteError {
    fn from(err: std::io::Error) -> Self {
        JuteError::Io(err)
    }
}

impl From<std::fmt::Error> for JuteError {
    fn from(err: std::fmt::Error) -> Self {
        JuteError::Fmt(err)
    }
}

impl From<std::string::FromUtf8Error> for JuteError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        JuteError::Utf8(err)
    }
}
