use std::{collections::HashSet, error::Error};

#[derive(Default, Debug)]
pub struct Doc {
    pub src: String,
    pub includes: Vec<String>,
    pub modules: Vec<Module>,
}

impl Doc {
    // it validate that this file alone doesn't include the same modules and include statement
    // doesn't contain module present in the file
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        let mut module_name_set = HashSet::new();
        for module in &self.modules {
            if let Err(e) = module.validate() {
                return Err(format!("{}, in file {}", e, self.src).into());
            }
            if !module_name_set.insert(&module.name) {
                return Err(format!(
                    "file : {}, contain same module name {} multiple times",
                    self.src, module.name
                )
                .into());
            }
        }
        // it is also invalid if we have to includes some module in the same files
        for included_module in &self.includes {
            if module_name_set.contains(included_module) {
                return Err(format!(
                    "file: {}, include module {}, which is declared in the same module",
                    self.src, included_module
                )
                .into());
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub records: Vec<Class>,
}

impl Module {
    // this will validate that the record is valid in its own
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        let mut record_name_set = HashSet::new();
        for class in &self.records {
            if let Err(e) = class.validate() {
                return Err(format!("{},  in module {}", e, self.name).into());
            }
            if !record_name_set.insert(&class.name) {
                return Err(format!(
                    "Module {}, conatain multiple records with same name {}",
                    self.name, class.name
                )
                .into());
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Class {
    // this will validate that the record is valid in its own
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        // to validate this we will just check if the field names are repeating or not
        let mut field_name_set = HashSet::new();
        for Field { name, _type } in &self.fields {
            if field_name_set.insert(name) {
                return Err(
                    format!("Record {}, contain repeated field name {}", self.name, name).into(),
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub _type: Type,
}

#[derive(Debug)]
pub enum Type {
    Boolean,
    Byte,
    Int,
    Long,
    Float,
    Double,
    UString,
    Buffer,
    Class { name: String, namespace: String },
    Vector(Box<Type>),
    Map(Box<Type>, Box<Type>),
}
