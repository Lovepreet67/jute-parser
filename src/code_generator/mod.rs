use std::{fs, path::Path};

use crate::errors::JuteError;

pub mod rust;

pub trait CodeGenerator {
    fn generate(&mut self) -> Result<String, JuteError>;
    fn write(&mut self, o_file: &Path) -> Result<(), JuteError> {
        let code = self.generate()?;
        fs::write(o_file, code)?;
        Ok(())
    }
}
