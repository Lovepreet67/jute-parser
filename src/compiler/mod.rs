use crate::compiler::{ast::Doc, parser::Parser};
use std::{fs, path::Path};

pub mod ast;
pub mod ast_printer;
pub mod lexer;
pub mod parser;
pub mod state_machine;
pub mod token;
pub mod validator;

pub fn build_ast(jute_file: &Path) -> Doc {
    // first we will read the file to string
    let source = fs::read_to_string(jute_file).unwrap();
    Parser::new("".to_string(), source)
        .parser()
        .expect("Error while generating ast")
}
