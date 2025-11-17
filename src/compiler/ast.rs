use std::rc::Rc;

pub struct Doc {
    includes: Vec<String>,
    modules: Vec<Modules>,
}
pub struct Modules {
    name: String,
    records: Vec<Record>,
}
pub struct Record {
    name: String,
    fields: Vec<Field>,
}
pub struct Field {
    name: String,
    _type: Type,
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
    Record { name: String, namespace: String },
    Vector(Rc<Type>),
    Map(Rc<Type>, Rc<Type>),
}
