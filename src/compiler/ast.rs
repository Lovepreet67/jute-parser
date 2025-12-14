use std::{collections::HashSet, path::PathBuf};

use crate::errors::JuteError;

#[derive(Default, Debug)]
pub struct Doc {
    pub src: PathBuf,
    pub includes: Vec<PathBuf>,
    pub modules: Vec<Module>,
}

impl Doc {
    // it checks if the single doc contain same modules more than one time and all modules are valid
    pub fn validate(&self) -> Result<(), JuteError> {
        let mut module_name_set = HashSet::new();
        for module in &self.modules {
            module.validate(&self.src)?;
            if !module_name_set.insert(&module.name) {
                return Err(JuteError::DuplicateModuleName {
                    module_name: module.name.clone(),
                    file_name: self.src.clone(),
                });
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
    // this will validate that the module is valid in its own (all records are valid and no record name repeated)
    pub fn validate(&self, file_src: &PathBuf) -> Result<(), JuteError> {
        let mut record_name_set = HashSet::new();
        for class in &self.records {
            class.validate(&self.name, file_src)?;
            if !record_name_set.insert(&class.name) {
                return Err(JuteError::DuplicateClassName {
                    class_name: class.name.clone(),
                    module_name: self.name.clone(),
                    file_name: file_src.clone(),
                });
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
    // this will validate that the record is valid in its own (field names are not repeated) field type validation is not done
    pub fn validate(&self, module_name: &str, file_name: &PathBuf) -> Result<(), JuteError> {
        // to validate this we will just check if the field names are repeating or not
        let mut field_name_set = HashSet::new();
        for Field { name, _type } in &self.fields {
            if field_name_set.insert(name) {
                return Err(JuteError::DuplicateFieldName {
                    field_name: name.clone(),
                    class_name: self.name.clone(),
                    module_name: module_name.to_string(),
                    file_name: file_name.clone(),
                });
            }
            // second check will be that the records in jute can't be recursive
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub _type: Type,
}

#[derive(Debug, Clone)]
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
