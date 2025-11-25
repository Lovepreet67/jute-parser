pub mod code_generator;
pub mod compiler;

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
