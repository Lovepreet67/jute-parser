pub mod rust;

pub trait CodeGenerator {
    fn generate(&mut self) -> String;
}
