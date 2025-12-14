use crate::{
    compiler::{ast::Doc, parser::Parser},
    errors::JuteError,
};
use std::{fs, path::Path};

pub mod ast;
pub mod ast_printer;
pub mod dependency_resolver;
pub mod lexer;
pub mod parser;
pub mod state_machine;
pub mod token;

pub fn build_ast(jute_file: &Path) -> Result<Doc, JuteError> {
    // first we will read the file to string
    let source = fs::read_to_string(jute_file).unwrap();
    Parser::new(jute_file.into(), source)?.parse()
}
