use crate::{
    code_generator::{
        CodeGenerator,
        rust::utilities::{get_field_name, get_field_type, get_module_name, get_record_name},
    },
    compiler::ast::{Class, Module},
};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Write,
    fs, io,
    path::PathBuf,
};

struct GeneratedModule {
    pub file_path: PathBuf,
    pub sub_modules: HashSet<String>,
    pub vector_imported: bool,
    pub hashset_imported: bool,
    pub struct_includes: HashSet<String>,
}
impl GeneratedModule {
    pub fn new(location: PathBuf) -> Self {
        Self {
            file_path: location,
            sub_modules: Default::default(),
            vector_imported: false,
            hashset_imported: false,
            struct_includes: Default::default(),
        }
    }
    pub fn insert_submodule(&mut self, submodule_name: &str) {
        println!("inside insert module");
        if !self.sub_modules.contains(submodule_name) {
            let mut file_writer = fs::OpenOptions::new()
                .append(true)
                .open(&self.file_path)
                .unwrap();
            let content = format!("pub mod {};\n", submodule_name.split("/").last().unwrap());
            io::Write::write_all(&mut file_writer, content.as_bytes()).unwrap();
            self.sub_modules.insert(submodule_name.to_string());
        }
    }
    pub fn insert_record(&mut self, record: &Class) -> Result<(), Box<dyn Error>> {
        let mut code = String::new();
        code.push_str("/*This is a auto generated code based on the jute file provided*/\n");
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
        let mut file_writer = fs::OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .unwrap();
        io::Write::write_all(&mut file_writer, code.as_bytes()).unwrap();
        Ok(())
    }
}

pub struct RustCodeGenerator {
    target_dir: String,
    modules: Vec<Module>,
    dependency: HashMap<String, String>,
    module_map: HashMap<String, GeneratedModule>,
}

impl RustCodeGenerator {
    pub fn new(
        modules: Vec<Module>,
        dependency: HashMap<String, String>,
        target_dir: String,
    ) -> Self {
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
        for module in &self.modules {
            let module_path: Vec<&str> = module.name.split(|c| c == '.').collect();
            for (i, module) in module_path.iter().enumerate() {
                let current_module = module_path[0..i + 1].join("/");
                if !self.module_map.contains_key(&current_module) {
                    // means module is already there we just check
                    let mut location = PathBuf::from(current_module.clone());
                    // first we will create a dir
                    fs::create_dir_all(&location).unwrap();
                    let mod_location = location.join("mod.rs");
                    fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(&mod_location)
                        .unwrap();
                    self.module_map
                        .insert(current_module.clone(), GeneratedModule::new(mod_location));
                }
                let parrent_module = module_path[0..i].join("/");
                if !parrent_module.is_empty() {
                    let generated_parrent_module =
                        self.module_map.get_mut(&parrent_module).unwrap();
                    generated_parrent_module.insert_submodule(&current_module);
                }
            }
            let curr_generated_module = self.module_map.get_mut(&module_path.join("/")).unwrap();
            // now we will import each struct in the module
            for record in &module.records {
                curr_generated_module.insert_record(record)?;
            }
        }
        Ok("".to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        code_generator::{
            CodeGenerator,
            rust::writer::{GeneratedModule, RustCodeGenerator},
        },
        compiler::build_ast,
    };
    use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
    };

    // #[test]
    // fn test_rust_code_generation() {
    //     let doc = build_ast(Path::new("./test.jute"));
    //     let mut code_generator = RustCodeGenerator::new(doc);
    //     code_generator.write(Path::new("./out.rs")).unwrap();
    //     eprintln!("code generated");
    // }

    #[test]
    fn test_debug() {
        let module = "a.b.c".to_string();
        let module_path: Vec<&str> = module.split(|c| c == '.').collect();
        let mut module_map = HashMap::new();
        for (i, module) in module_path.iter().enumerate() {
            let current_module = module_path[0..i + 1].join("/");
            println!("current module : {}", current_module);
            if !module_map.contains_key(&current_module) {
                // means module is already there we just check
                let mut location = PathBuf::from(current_module.clone());
                // first we will create a dir
                fs::create_dir_all(&location).unwrap();
                let mod_location = location.join("mod.rs");
                println!("location with mod : {:?}", mod_location);
                fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&mod_location)
                    .unwrap();
                module_map.insert(current_module.clone(), GeneratedModule::new(mod_location));
            }
            let parrent_module = module_path[0..i].join("/");
            if !parrent_module.is_empty() {
                let generated_parrent_module = module_map.get_mut(&parrent_module).unwrap();
                generated_parrent_module.insert_submodule(&current_module);
            }
            println!("parrent module : {}", parrent_module);
        }
    }
}
