use std::{fmt, path::PathBuf};

use crate::compiler::{ast::Type, token::Token};

#[derive(Debug)]
pub enum TypeValidationError {
    UnknownType,
    AmbigousType,
}

#[derive(Debug)]
#[non_exhaustive]
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

impl fmt::Display for JuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JuteError::UnexpectedChar { c, message } => {
                write!(f, "unexpected character '{}': {}", c, message)
            }

            JuteError::UnexpectedToken { token, message } => {
                write!(f, "unexpected token {:?}: {}", token, message)
            }

            JuteError::UnknownType {
                name,
                record,
                module,
                file,
                ..
            } => write!(
                f,
                "unknown type '{}' in record '{}' (module '{}') at {:?}",
                name, record, module, file
            ),

            JuteError::AmbiguousType {
                name,
                record,
                module,
                file,
                ..
            } => write!(
                f,
                "ambiguous type '{}' in record '{}' (module '{}') at {:?}",
                name, record, module, file
            ),

            JuteError::CircularDependency { cycle } => {
                write!(f, "circular dependency detected: {}", cycle)
            }

            JuteError::Io(e) => write!(f, "io error: {}", e),
            JuteError::Fmt(e) => write!(f, "formatting error: {}", e),

            _ => write!(f, "{:?}", self),
        }
    }
}
