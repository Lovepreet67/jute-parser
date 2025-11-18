use std::rc::Rc;

#[derive(Default)]
pub struct Doc {
    pub includes: Vec<String>,
    pub modules: Vec<Module>,
}
pub struct Module {
    pub name: String,
    pub records: Vec<Class>,
}
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
}
pub struct Field {
    pub name: String,
    pub _type: Type,
}

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
    Vector(Rc<Type>),
    Map(Rc<Type>, Rc<Type>),
}
