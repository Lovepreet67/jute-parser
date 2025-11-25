use crate::{code_generator::CodeGenerator, compiler::ast::Doc};

pub struct RustCodeGenerator {
    doc: Doc,
}

impl RustCodeGenerator {
    pub fn new(doc: Doc) -> Self {
        Self { doc }
    }
}

impl CodeGenerator for RustCodeGenerator {
    fn generate(&mut self) -> String {
        let mut code = String::new();
        code.push_str("/*This is a auto generated code based on the jute file provided*/\n");
        code
    }
}
