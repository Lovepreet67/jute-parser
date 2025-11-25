use crate::compiler::ast::{Class, Doc, Field, Module, Type};

pub struct AstPrinter {
    indent: usize,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    fn print_indent(&self) {
        for _ in 0..self.indent {
            print!("    ");
        }
    }

    fn line(&self, label: impl AsRef<str>) {
        self.print_indent();
        println!("{}", label.as_ref());
    }

    fn with_indent<F: FnOnce(&mut AstPrinter)>(&mut self, f: F) {
        self.indent += 1;
        f(self);
        self.indent -= 1;
    }

    pub fn print_doc(&mut self, doc: &Doc) {
        self.line("Doc");

        // includes
        self.with_indent(|p| {
            p.line("includes:");
            p.with_indent(|p| {
                for inc in &doc.includes {
                    p.line(format!("{:?}", inc));
                }
            });

            p.line("modules:");
            p.with_indent(|p| {
                for m in &doc.modules {
                    p.print_module(m);
                }
            });
        });
    }

    fn print_module(&mut self, m: &Module) {
        self.line(format!("Module: {}", m.name));

        self.with_indent(|p| {
            for c in &m.records {
                p.print_class(c);
            }
        });
    }

    fn print_class(&mut self, c: &Class) {
        self.line(format!("Class: {}", c.name));

        self.with_indent(|p| {
            for f in &c.fields {
                p.print_field(f);
            }
        });
    }

    fn print_field(&mut self, f: &Field) {
        self.line(format!("Field: {} -> {}", f.name, ptype(&f._type)));
    }
}

fn ptype(t: &Type) -> String {
    match t {
        Type::Boolean => "Boolean".into(),
        Type::Byte => "Byte".into(),
        Type::Int => "Int".into(),
        Type::Long => "Long".into(),
        Type::Float => "Float".into(),
        Type::Double => "Double".into(),
        Type::UString => "UString".into(),
        Type::Buffer => "Buffer".into(),
        Type::Class { name, namespace } => format!("Class({}::{})", namespace, name),
        Type::Vector(inner) => format!("Vector<{}>", ptype(inner)),
        Type::Map(k, v) => format!("Map<{}, {}>", ptype(k), ptype(v)),
    }
}
