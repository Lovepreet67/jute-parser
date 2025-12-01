use crate::compiler::build_ast;
use std::{error::Error, path::Path};

pub mod code_generator;
pub mod compiler;

#[derive(Default)]
pub struct JuteGenerator {
    pub out_dir: Option<String>,
    pub src_files: Vec<String>,
}
impl JuteGenerator {
    pub fn new() -> Self {
        Self {
            out_dir: None,
            src_files: Vec::new(),
        }
    }
    pub fn add_src_file(mut self, file_path: String) -> Self {
        if self.src_files.iter().any(|x| **x == file_path) {
            panic!("Same file added multiple times");
        }
        self.src_files.push(file_path);
        self
    }
    pub fn add_out_dir(mut self, out_dir: String) -> Self {
        if self.out_dir.is_some() {
            panic!("Output dir alreay assigned");
        }
        self.out_dir = Some(out_dir);
        self
    }
    pub fn generate(self) -> Result<(), Box<dyn Error>> {
        // this function will handle all the generation logic
        // we will parse all the files one by one and push docs in the vector
        let mut docs = Vec::new();
        for file in self.src_files {
            let doc = build_ast(Path::new(&file));
            docs.push(doc);
        }
        // now we have a array of docs now we will validate this doc
        todo!();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::{ast_printer::AstPrinter, parser::Parser};
    use std::fs;
    #[test]
    fn root_test() {
        // first we will read the file to string
        let source = fs::read_to_string("./test.jute").unwrap();
        println!("source : {}", source);
        let ast = Parser::new("".to_string(), source)
            .parser()
            .expect("Error while generating ast");
        println!("Generated ast successfully");
        let mut ast_printer = AstPrinter::new();
        ast_printer.print_doc(&ast);
    }
}
