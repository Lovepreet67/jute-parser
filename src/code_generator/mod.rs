use std::{fs, path::Path};

use crate::errors::JuteError;

pub mod rust;

pub(crate) trait CodeGenerator {
    fn generate(&mut self) -> Result<String, JuteError>;
    #[warn(dead_code)]
    fn write(&mut self, o_file: &Path) -> Result<(), JuteError> {
        let code = self.generate()?;
        fs::write(o_file, code)?;
        Ok(())
    }
}
