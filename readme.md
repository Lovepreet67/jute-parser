# Jute Parser & Rust Code Generator

A **Rust** implementation of the Apache ZooKeeper Jute IDL, providing a compiler-grade solution for protocol schema parsing and code generation.

This crate is designed for **compiler-grade correctness**, not just simple code generation, ensuring your generated code is based on a fully validated schema.

## ✨ Features

* **📜 Parsing:** Full Jute grammar support, including `module`, `class`, primitive types, `vector<T>`, `map<K, V>`, and cross-module type references.
* **🔗 Cross-File Resolution:** Robust handling of relative file includes (`include "./path.jute";`).
* **🧠 Dependency Validation:**
    * Dependency graph resolution.
    * Detects unknown types and ambiguous references.
    * Detects **circular dependencies**.
* **🦀 Rust Code Generation:**
    * Idiomatic Rust output.
    * Read/Write based serialization.
    * No runtime reflection.
* **🔄 Serialization & Deserialization:** Provides high-performance, strongly-typed serialization logic.
* **🧪 Strong Test Coverage:** Comprehensive tests for the parser, resolver, and serialization round-trips.

## 🚀 Quick Start

### 1. Example Jute Schema

Create a schema file, e.g., `schema/serialization.jute`:

```jute
module serialization {
    class Person {
        int id;
        string name;
        boolean active;
    }

    class Company {
        int id;
        Person owner;
        vector<Person> employees;
    }
}
```

### 2. Generate Rust Code

Use the `JuteGenerator` to process your schema and generate source files:

```rust
use jute::JuteGenerator;

JuteGenerator::new()
    .add_src_file("schema/serialization.jute") // Input schema
    .add_out_dir("generated") // Output directory src (curr crate) will be used as root 
    .generate()
    .unwrap();
```

### 3. Using Generated Code

The generated code is not automatically wired into your crate. You must include it manually:

```rust
// src/lib.rs or src/main.rs
pub mod serialization {
    include!("generated/serialization/mod.rs");
}
```

### 4. 🔄 Serialization & Deserialization

All generated types implement two core methods for I/O-based serialization:
```Rust

fn serialize<W: std::io::Write>(&self, out: &mut W) -> Result<(), JuteError>;
fn deserialize<R: std::io::Read>(input: &mut R) -> Result<Self, JuteError>;
```



**Example Round-Trip:**

```rust
use crate::serialization::{Person, Company};

let person = Person {
    id: 1,
    name: "Alice".into(),
    active: true,
};

let mut buffer = Vec::new();
// Serialize
person.serialize(&mut buffer).unwrap();

// Deserialize
let decoded = Person::deserialize(&mut &buffer[..]).unwrap();

assert_eq!(decoded.id, 1);
// ... more assertions
```
A complete end-to-end example demonstrating how to use this crate is available here:**[View the full E2E example](examples/e2e/README.md)**

## 📂 Include Semantics

Includes are resolved **relative** to the file that contains the include directive:

```jute
include "./common.jute";
include "../shared/types.jute";
```
The resolver ensures all paths are:

* **Canonicalized**
* **Deduplicated**
* **Cycle-checked** 

---

## ❌ Error Handling

All failures return a unified, descriptive error type:

```rust
use jute::errors::JuteError;
```
Errors are descriptive and contextual, including file paths and module names. They cover:
* Syntax errors in the schema
* Unknown or ambiguous types
* Circular dependencies
* IO failures
* Invalid schema definitions

---

## 🏗 Project Structure

The project follows a classic compiler architecture:
```
src/
├── compiler/
│   ├── ast_printer.rs // utility
│   ├── ast.rs              
│   ├── dependency_resolver.rs
│   ├── lexer.rs 
│   ├── parser.rs  
│   ├── state_machine.rs  
│   ├── token.rs   
│   ├── mod.rs     
├── code_generator/
│   └── rust/    
│   |   └── utilities.rs
│   |   └── writer.rs
│   |   └── mod.rs
│   └── mod.rs    
├── errors.rs
└── lib.rs
```

## 🎯 Design Goals

This crate prioritizes stability and correctness:

* **Correctness > Convenience**
* Explicit over implicit
* No hidden build steps
* Deterministic code generation

It is intended for:

* Zookepeer adapters
* Protocol tooling
* Schema-driven serialization

## 🧠 Inspiration

* Apache ZooKeeper Jute 
* Protocol Buffers (IDL-style)

---

## Missing Implementation
Following features are currently not supported
* Containerized recursive schema
* Can be enhanced with async versions of serialization.


## 📜 License

This project is licensed under the **MIT License**.


## 🤝 Contributing

Contributions are welcome! Focus areas include:

* Adding new Jute features
* Improving error diagnostics
* Extending code generators (e.g., for other languages)
* Improving test coverage
* Implementing missing features
* benchmarking

