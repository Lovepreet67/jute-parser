use crate::{
    code_generator::{CodeGenerator, rust::writer::RustCodeGenerator},
    compiler::{ast::Module, build_ast, dependency_resolver::resolve_dependencies},
};
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
        let dependencies = resolve_dependencies(&docs)?;
        let merged_modules: Vec<Module> =
            docs.into_iter().map(|doc| doc.modules).flatten().collect();
        // now we have a array of docs now we will validate this doc
        RustCodeGenerator::new(
            merged_modules,
            dependencies,
            self.out_dir.unwrap_or("".to_string()),
        )
        .generate();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn root_test() {
        // first we will read the file to string
        let jg = JuteGenerator::new();
        jg.add_src_file("./jute_test_schema/test1.jute".to_string())
            .add_src_file("./jute_test_schema/test2.jute".to_string())
            .generate()
            .unwrap();
    }
}
