use std::collections::HashSet;

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
