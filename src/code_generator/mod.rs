use std::{error::Error, fs, path::Path};

pub mod rust;

pub trait CodeGenerator {
    fn generate(&mut self) -> Result<String, Box<dyn Error>>;
    fn write(&mut self, o_file: &Path) -> Result<(), Box<dyn Error>> {
        let code = self.generate()?;
        fs::write(o_file, code)?;
        Ok(())
    }
}
