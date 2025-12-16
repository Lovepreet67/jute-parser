use std::{fmt, path::PathBuf};

use std::error::Error;

#[derive(Debug)]
pub(crate) enum TypeValidationError {
    UnknownType,
    AmbigousType,
}

/// Top-level error type for the Jute compiler and code generator.
/// `JuteError` represents all possible failures that can occur during:
/// - Lexing
/// - Parsing
/// - Dependency resolution
/// - Type validation
/// - Code generation
/// - I/O operations
/// This enum is marked as `#[non_exhaustive]` to allow adding new error
/// variants in the future without breaking downstream code.
/// Consumers should match using a wildcard arm (`_`) to remain forward-compatible.
#[derive(Debug)]
#[non_exhaustive]
pub enum JuteError {
    /// An invalid conversion was attempted from a token to a primitive type.
    /// This usually indicates a invalid token where permetive type was excepted
    InvalidConversionToPremitive {
        /// The token that failed conversion.
        token: String,
    },
    /// An unexpected token was encountered during parsing.
    /// This typically means the input does not conform to the Jute grammar.
    UnexpectedToken {
        /// The token that was encountered (if any).
        token: Option<String>,
        /// Human-readable explanation of what was expected.
        message: String,
    },
    /// An unexpected character was encountered during lexing.
    UnexpectedChar {
        /// The unexpected character.
        c: char,
        /// Additional context about the error.
        message: String,
    },
    // The token stream ended unexpectedly.
    /// This usually indicates malformed or truncated input.
    UnexpectedEndOfTokenStream,
    /// The end of the source file was reached unexpectedly.
    /// Common causes include missing braces or incomplete declarations.
    UnexpectedEndOfFile,
    /// Failed to canonicalize a filesystem path.
    ///
    /// This can occur when resolving include paths or source file locations.
    PathCanonicalizeError {
        /// Additional context explaining the failure.
        message: String,
    },
    /// A multiline comment (`/* ... */`) was not properly terminated.
    UnTerminatedMultiLineComment,
    /// A quoted string literal was not properly terminated.
    UnTerminatedQuotedString,
    /// A quoted string literal contains a newline character,
    QuotedStringWithNewLineCharacter,

    /// A referenced type could not be resolved.
    /// Provides detailed context to help diagnose schema issues.
    UnknownType {
        /// Field name
        name: String,
        /// Type
        _type: String,
        /// Parrent record/class
        record: String,
        /// Parrent module
        module: String,
        /// Src File
        file: PathBuf,
    },
    // A referenced type name resolved to multiple definitions.
    /// This usually happens when two included modules define
    /// the same type name.
    AmbiguousType {
        /// The ambiguous type name.
        name: String,
        /// The expected kind of the type.
        _type: String,
        /// Record in which the ambiguity occurred.
        record: String,
        /// Module containing the record.
        module: String,
        /// Source file where the error occurred.
        file: PathBuf,
    },
    /// A circular dependency was detected between modules.
    /// The `cycle` field describes the dependency loop.
    CircularDependency {
        /// Human-readable representation of the dependency cycle.
        cycle: String,
    },
    /// Wrapper around [`std::io::Error`].
    /// Used for filesystem and I/O related failures.
    Io(std::io::Error),
    /// Wrapper around [`std::fmt::Error`].
    ///
    /// Typically occurs during code generation.
    Fmt(std::fmt::Error),
    /// Wrapper around UTF-8 decoding errors.
    Utf8(std::string::FromUtf8Error),
    /// Duplicate module names detected across source files.
    DuplicateModuleName {
        /// Name of the duplicated module.
        module_name: String,
        /// File in which the duplicate was found.
        file_name: PathBuf,
    },
    /// Duplicate class names detected within the same module.
    DuplicateClassName {
        /// Name of the duplicated class.
        class_name: String,
        /// Module containing the duplicate.
        module_name: String,
        /// File in which the duplicate was found.
        file_name: PathBuf,
    },

    /// Duplicate field names detected within a class.
    DuplicateFieldName {
        /// Name of the duplicated field.
        field_name: String,
        /// Class containing the duplicate field.
        class_name: String,
        /// Module containing the class.
        module_name: String,
        /// File in which the duplicate was found.
        file_name: PathBuf,
    },
    /// An invalid field type was used in a class definition.
    InvalidFieldType {
        /// The invalid field type.
        field_type: String,
        /// Field name.
        field_name: String,
        /// Class containing the field.
        class_name: String,
        /// Module containing the class.
        module_name: String,
        /// File in which the error occurred.
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

impl Error for JuteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            JuteError::Io(e) => Some(e),
            JuteError::Fmt(e) => Some(e),
            JuteError::Utf8(e) => Some(e),
            _ => None,
        }
    }
}
