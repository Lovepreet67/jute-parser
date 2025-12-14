use crate::{
    code_generator::{
        CodeGenerator,
        rust::utilities::{get_field_name, get_record_name},
    },
    compiler::ast::{Class, Module, Type},
    errors::JuteError,
};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Write,
    fs, io,
    path::PathBuf,
};

struct GeneratedModule {
    pub name: String,
    pub file_path: PathBuf,
    pub sub_modules: HashSet<String>,
}
impl GeneratedModule {
    pub fn new(name: String, location: PathBuf) -> Self {
        Self {
            name,
            file_path: location,
            sub_modules: Default::default(),
        }
    }
    // TODO: This function signature should be changed
    pub fn get_field_type(
        &mut self,
        jute_type: &Type,
        dependency_map: &HashMap<String, String>,
        target_dir: &str,
        record_name: &str,
    ) -> Cow<'static, str> {
        match jute_type {
            Type::Int => Cow::Borrowed("i32"),
            Type::Long => Cow::Borrowed("i64"),
            Type::Float => Cow::Borrowed("f32"),
            Type::Double => Cow::Borrowed("f64"),
            Type::Boolean => Cow::Borrowed("bool"),
            Type::Byte => Cow::Borrowed("u8"),
            Type::Buffer => Cow::Borrowed("std::vec::Vec<u8>"),
            Type::UString => Cow::Borrowed("String"),
            Type::Class { name, namespace } => {
                let resloved_path = dependency_map
                    .get(&format!(
                        "{}.{}.{}.{}",
                        self.name, record_name, namespace, name
                    ))
                    .unwrap();
                let mut path = format!("jute");
                if !target_dir.is_empty() {
                    path.push_str("::");
                    path.push_str(target_dir);
                }
                for item in resloved_path.split(".") {
                    path.push_str("::");
                    path.push_str(item);
                }
                Cow::Owned(path)
            }
            Type::Vector(t) => Cow::Owned(format!(
                "std::vec::Vec<{}>",
                self.get_field_type(t, dependency_map, target_dir, record_name)
            )),
            Type::Map(t1, t2) => Cow::Owned(format!(
                "std::collections::Hashmap<{},{}>",
                self.get_field_type(t1, dependency_map, target_dir, record_name),
                self.get_field_type(t2, dependency_map, target_dir, record_name)
            )),
        }
    }
    pub fn insert_submodule(&mut self, submodule_name: &str) {
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
    pub fn insert_record(
        &mut self,
        record: &Class,
        dependency_map: &HashMap<String, String>,
        target_dir: &str,
    ) -> Result<(), JuteError> {
        let mut code = String::new();
        code.push_str("/*This is a auto generated code based on the jute file provided*/\n");
        writeln!(
            code,
            "#[derive(Default,Clone)]\n\tpub struct {} {{",
            get_record_name(&record.name)
        )?;
        for field in &record.fields {
            writeln!(
                code,
                "\tpub {} : {},",
                get_field_name(&field.name),
                self.get_field_type(&field._type, dependency_map, target_dir, &record.name)
            )?
        }
        writeln!(code, "}}")?;
        // writing getters and setters for this
        writeln!(code, "impl {} {{", get_record_name(&record.name))?;
        let mut jute_field_to_rust = HashMap::new();
        for field in &record.fields {
            let normalized_field_name = get_field_name(&field.name);
            let field_type =
                self.get_field_type(&field._type, dependency_map, target_dir, &record.name);
            jute_field_to_rust.insert(&field.name, (normalized_field_name, field_type));
            writeln!(
                code,
                "\tpub fn get_{}(&self)->{}{{\n\t\t\tself.{}.clone()\n\t\t}}",
                jute_field_to_rust.get(&field.name).unwrap().0,
                jute_field_to_rust.get(&field.name).unwrap().1,
                jute_field_to_rust.get(&field.name).unwrap().0
            )?;
            writeln!(
                code,
                "\tpub fn set_{}(&mut self,val: {}){{\n\t\t\tself.{} = val\n\t\t}}",
                jute_field_to_rust.get(&field.name).unwrap().0,
                jute_field_to_rust.get(&field.name).unwrap().1,
                jute_field_to_rust.get(&field.name).unwrap().0
            )?;
        }
        writeln!(code, "}}")?;
        // writing serialization and deserialization code
        writeln!(
            code,
            "impl jute::code_generator::rust::JuteSerializable for {} {{",
            get_record_name(&record.name)
        )?;
        writeln!(
            code,
            "\tfn serailaize<W:Write>(&self, out:&mut W)->Result<(),Box<dyn Error>> {{"
        )?;
        for field in &record.fields {
            writeln!(
                code,
                "\t\tself.{}.serialize(out)?;",
                jute_field_to_rust.get(&field.name).unwrap().0
            )?;
        }
        writeln!(code, "\t\t}}")?;
        writeln!(
            code,
            "\tfn deserailaize<R:Read>( out:&mut R)->Result<Self,Box<dyn Error>> {{"
        )?;
        writeln!(code, "\t\tSelf {{")?;
        for field in &record.fields {
            writeln!(
                code,
                "\t\t\t {} : {}::deserialize(bytes)?,",
                jute_field_to_rust.get(&field.name).unwrap().0,
                jute_field_to_rust.get(&field.name).unwrap().1,
            )?;
        }
        writeln!(code, "\t\t}}")?;
        writeln!(code, "\t}}")?;

        writeln!(code, "}}")?;
        let mut file_writer = fs::OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .unwrap();
        io::Write::write_all(&mut file_writer, code.as_bytes()).unwrap();
        Ok(())
    }
}

pub struct RustCodeGenerator {
    target_dir: PathBuf,
    modules: Vec<Module>,
    dependency: HashMap<String, String>,
    module_map: HashMap<String, GeneratedModule>,
}

impl RustCodeGenerator {
    pub fn new(
        modules: Vec<Module>,
        dependency: HashMap<String, String>,
        target_dir: PathBuf,
    ) -> Self {
        Self {
            modules,
            dependency,
            target_dir: target_dir,
            module_map: HashMap::default(),
        }
    }
}

impl CodeGenerator for RustCodeGenerator {
    fn generate(&mut self) -> Result<String, JuteError> {
        for module in &self.modules {
            let module_path: Vec<&str> = module.name.split(|c| c == '.').collect();
            for (i, module) in module_path.iter().enumerate() {
                let current_module = module_path[0..i + 1].join("/");
                if !self.module_map.contains_key(&current_module) {
                    // means module is already there we just check
                    let mut location = PathBuf::from("src");
                    location.push(&self.target_dir);
                    location.push(&current_module);
                    // first we will create a dir
                    fs::create_dir_all(&location).unwrap();
                    let mod_location = location.join("mod.rs");
                    fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(&mod_location)
                        .unwrap();
                    self.module_map.insert(
                        current_module.clone(),
                        GeneratedModule::new(module_path[0..i + 1].join("."), mod_location),
                    );
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
                curr_generated_module.insert_record(
                    record,
                    &self.dependency,
                    &self.target_dir.to_str().unwrap(), // as we know path will be valid utf-8
                )?;
            }
        }
        Ok("".to_string())
    }
}
