#[derive(Default, Debug)]
pub struct Doc {
    pub includes: Vec<String>,
    pub modules: Vec<Module>,
}
#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub records: Vec<Class>,
}
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
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
