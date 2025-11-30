use crate::compiler::ast::Type;
use std::{borrow::Cow, collections::HashSet};

fn camle_case_to_snake_case(original: &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if c.is_uppercase() {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

fn rust_keyword_mapper(original: &str) -> String {
    let rust_keywords: HashSet<&'static str> = HashSet::from([
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
    ]);

    if rust_keywords.contains(original) {
        format!("{}_", original)
    } else {
        original.to_string()
    }
}

pub fn get_module_name(original_name: &str) -> String {
    rust_keyword_mapper(original_name)
}

pub fn get_record_name(original_name: &str) -> String {
    rust_keyword_mapper(original_name)
}

pub fn get_field_name(original_name: &str) -> String {
    camle_case_to_snake_case(&rust_keyword_mapper(original_name))
}

pub fn get_field_type(jute_type: &Type) -> Cow<'static, str> {
    match jute_type {
        Type::Int => Cow::Borrowed("i32"),
        Type::Long => Cow::Borrowed("i64"),
        Type::Float => Cow::Borrowed("f32"),
        Type::Double => Cow::Borrowed("f64"),
        Type::Boolean => Cow::Borrowed("bool"),
        Type::Byte => Cow::Borrowed("u8"),
        Type::Buffer => Cow::Borrowed("std::vec::Vec<u8>"),
        Type::UString => Cow::Borrowed("String"),
        Type::Class { name, namespace: _ } => Cow::Owned(name.clone()),
        Type::Vector(t) => Cow::Owned(format!("std::vec::Vec<{}>", get_field_type(t))),
        Type::Map(t1, t2) => Cow::Owned(format!(
            "std::collections::Hashmap<{},{}>",
            get_field_type(t1),
            get_field_type(t2)
        )),
    }
}
