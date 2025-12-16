//! # Jute
//!
//! Jute is a **schema-driven code generator** for Rust, inspired by
//! Apache ZooKeeper’s *Jute* serialization format.
//!
//! This crate provides a high-level API to:
//!
//! - Parse `.jute` schema files into an abstract syntax tree (AST)
//! - Resolve cross-file and cross-module type dependencies
//! - Validate schemas (unknown types, ambiguities, cycles, etc.)
//! - Generate idiomatic, type-safe Rust code
//! - Generate binary serialization and deserialization logic
//!
//! The primary entry point to the API is [`JuteGenerator`], which
//! orchestrates the full pipeline from schema parsing to code generation.
//!
//! ## Typical Workflow
//!
//! 1. Author one or more `.jute` schema files
//! 2. Feed them into [`JuteGenerator`]
//! 3. Generate Rust code into a target directory
//! 4. Compile and use the generated types directly in your application
//!
//! ## Example
//!
//! ```no_run
//! use jute::JuteGenerator;
//!
//! JuteGenerator::new()
//!     .add_src_file("schema/common.jute")
//!     .add_src_file("schema/model.jute")
//!     .add_out_dir("generated")
//!     .generate()
//!     .expect("Jute code generation failed");
//! ```
//!

use crate::{
    code_generator::{CodeGenerator, rust::writer::RustCodeGenerator},
    compiler::{ast::Module, build_ast, dependency_resolver::resolve_dependencies},
};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

pub mod code_generator;
pub(crate) mod compiler;
pub mod errors;
pub use crate::code_generator::rust::JuteSerializable;
pub use crate::errors::JuteError;

/// High-level Jute code generation driver.
///
/// `JuteGenerator` follows a builder-style API and is responsible for:
/// 1. Collecting source `.jute` files
/// 2. Parsing them into ASTs
/// 3. Resolving type and module dependencies
/// 4. Invoking the language-specific code generator (Rust)
///
/// # Example
///
/// ```no_run
/// use jute::JuteGenerator;
/// let schema_path1 = "./schema/1.jute";
/// let schema_path2 = "./schema/2.jute";
/// let out_dir_path= "generated";
///
/// JuteGenerator::new()
///     .add_src_file(schema_path1)
///     .add_src_file(schema_path2)
///     .add_out_dir(out_dir_path)
///     .generate()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct JuteGenerator {
    /// Output directory where generated code will be written.
    ///
    /// If not provided, code generation defaults to the current(src) directory.
    pub out_dir: Option<PathBuf>,
    /// List of input Jute schema files.
    pub src_files: Vec<PathBuf>,
}
impl JuteGenerator {
    /// Creates a new empty `JuteGenerator`.
    pub fn new() -> Self {
        Self {
            out_dir: None,
            src_files: Vec::new(),
        }
    }

    /// Adds a Jute source file to the generator.
    ///
    /// # Panics
    ///
    /// Panics if the same file is added more than once.
    pub fn add_src_file<P: AsRef<Path>>(mut self, file_path: P) -> Self {
        let file_path = file_path.as_ref().to_path_buf().canonicalize().unwrap();
        if self.src_files.iter().any(|x| **x == file_path) {
            panic!("Same file added multiple times");
        }
        self.src_files.push(file_path);
        self
    }
    /// Sets the output directory for generated code.
    ///
    /// # Panics
    ///
    /// Panics if the output directory is already set.
    pub fn add_out_dir<P: AsRef<Path>>(mut self, out_dir: P) -> Self {
        if self.out_dir.is_some() {
            panic!("Output dir alreay assigned");
        }
        let out_dir = out_dir.as_ref().to_path_buf();
        self.out_dir = Some(out_dir);
        self
    }

    /// Executes the full code generation pipeline.
    ///
    /// This method:
    /// 1. Parses all provided `.jute` files into AST documents
    /// 2. Resolves cross-module and cross-file dependencies
    /// 3. Merges all modules into a single collection
    /// 4. Generates Rust code into the configured output directory
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Parsing fails
    /// - Dependency resolution fails
    /// - Code generation fails
    pub fn generate(self) -> Result<(), JuteError> {
        // this function will handle all the generation logic
        // we will parse all the files one by one and push docs in the vector
        let mut docs = Vec::new();
        for file in self.src_files {
            let doc = build_ast(Path::new(&file))?;
            docs.push(doc);
        }
        let dependencies = resolve_dependencies(&docs)?;
        let merged_modules: Vec<Module> =
            docs.into_iter().map(|doc| doc.modules).flatten().collect();
        // now we have a array of docs now we will validate this doc
        RustCodeGenerator::new(
            merged_modules,
            dependencies,
            self.out_dir.unwrap_or(PathBuf::from_str("").unwrap()), // unwrap will be never called
        )
        .generate()?;
        Ok(())
    }
}
