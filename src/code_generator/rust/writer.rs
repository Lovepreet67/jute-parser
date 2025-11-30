use crate::{
    code_generator::{
        CodeGenerator,
        rust::utilities::{get_field_name, get_field_type, get_module_name, get_record_name},
    },
    compiler::ast::Doc,
};
use std::{error::Error, fmt::Write};

pub struct RustCodeGenerator {
    doc: Doc,
}

impl RustCodeGenerator {
    pub fn new(doc: Doc) -> Self {
        Self { doc }
    }
}

impl CodeGenerator for RustCodeGenerator {
    fn generate(&mut self) -> Result<String, Box<dyn Error>> {
        let mut code = String::new();
        code.push_str("/*This is a auto generated code based on the jute file provided*/\n");
        for module in &self.doc.modules {
            writeln!(code, "pub mod {} {{", get_module_name(&module.name))?;
            for record in &module.records {
                writeln!(
                    code,
                    "\t#[derive(Default,Clone)]\n\tpub struct {} {{",
                    get_record_name(&record.name)
                )?;
                for field in &record.fields {
                    writeln!(
                        code,
                        "\t\tpub {} : {},",
                        get_field_name(&field.name),
                        get_field_type(&field._type)
                    )?
                }
                writeln!(code, "\t}}")?;
                // writing getters and setters for this
                writeln!(code, "\timpl {} {{", get_record_name(&record.name))?;
                for field in &record.fields {
                    writeln!(
                        code,
                        "\t\tpub fn get_{}(&self)->{}{{\n\t\t\tself.{}.clone()\n\t\t}}",
                        get_field_name(&field.name),
                        get_field_type(&field._type),
                        get_field_name(&field.name)
                    )?;
                    writeln!(
                        code,
                        "\t\tpub fn set_{}(&mut self,val: {}){{\n\t\t\tself.{} = val\n\t\t}}",
                        get_field_name(&field.name),
                        get_field_type(&field._type),
                        get_field_name(&field.name)
                    )?;
                }
                writeln!(code, "\t}}")?;
                // writing serialization and deserialization code
                writeln!(
                    code,
                    "\timpl JuteSerializable for {} {{",
                    get_record_name(&record.name)
                )?;
                writeln!(
                    code,
                    "\t\tfn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {{"
                )?;
                for field in &record.fields {
                    writeln!(
                        code,
                        "\t\t\tself.{}.serialize(out)?;",
                        get_field_name(&field.name)
                    )?;
                }
                writeln!(code, "\t\t}}")?;
                writeln!(
                    code,
                    "\t\tfn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {{"
                )?;
                writeln!(code, "\t\t\tSelf {{")?;
                for field in &record.fields {
                    writeln!(
                        code,
                        "\t\t\t\t {} : {}::deserialize(bytes)?,",
                        get_field_name(&field.name),
                        get_field_type(&field._type)
                    )?;
                }
                writeln!(code, "\t\t\t}}")?;
                writeln!(code, "\t\t}}")?;

                writeln!(code, "\t}}")?;
            }
            writeln!(code, "}}")?;
        }
        Ok(code)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        code_generator::{CodeGenerator, rust::writer::RustCodeGenerator},
        compiler::build_ast,
    };
    use std::path::Path;

    #[test]
    fn test_rust_code_generation() {
        let doc = build_ast(Path::new("./test.jute"));
        let mut code_generator = RustCodeGenerator::new(doc);
        code_generator.write(Path::new("./out.rs")).unwrap();
        eprintln!("code generated");
    }
}
