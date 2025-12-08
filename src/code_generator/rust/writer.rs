use crate::{
    code_generator::{
        CodeGenerator,
        rust::utilities::{get_field_name, get_field_type, get_module_name, get_record_name},
    },
    compiler::ast::{Doc, Module},
};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Write,
};

struct GeneratedModule {
    pub sub_modules: HashSet<String>,
    pub vector_imported: bool,
    pub hashset_imported: bool,
    pub struct_includes: bool,
}

pub struct RustCodeGenerator {
    target_dir: String,
    modules: Vec<Module>,
    dependency: HashMap<String, String>,
    module_map: HashMap<String, GeneratedModule>,
}

impl RustCodeGenerator {
    pub fn new(docs: Vec<Module>, dependency: HashMap<String, String>, target_dir: String) -> Self {
        Self {
            modules,
            dependency,
            target_dir,
            module_map: HashMap::default(),
        }
    }
}

impl CodeGenerator for RustCodeGenerator {
    fn generate(&mut self) -> Result<String, Box<dyn Error>> {
        for module in self.modules {
            let module_path: Vec<&str> = module.name.split(|c| c == '.').collect();
            for (i, module) in module_path.iter().enumerate() {
                let current_module = module_path[0..i].join("/");
                if self.module_map.contains_key(&current_module) {
                    // means module is already there we just check
                }
            }
        }
        Ok("".to_string())
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
