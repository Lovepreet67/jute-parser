use crate::{
    compiler::{ast::Doc, parser::Parser},
    errors::JuteError,
};
use std::{fs, path::Path};

pub(crate) mod ast;
pub(crate) mod ast_printer;
pub(crate) mod dependency_resolver;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod state_machine;
pub(crate) mod token;

pub(crate) fn build_ast(jute_file: &Path) -> Result<Doc, JuteError> {
    // first we will read the file to string
    let source = fs::read_to_string(jute_file).unwrap();
    Parser::new(jute_file.into(), source)?.parse()
}
