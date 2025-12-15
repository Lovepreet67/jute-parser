use jute::{JuteGenerator, errors::JuteError};
use tempdir::TempDir;

#[test]
fn parser_valid_basic() {
    let target_dir = TempDir::new("test").unwrap();

    JuteGenerator::new()
        .add_src_file("tests/parser/valid/basic.jute")
        .add_out_dir(target_dir)
        .generate()
        .unwrap();
}

#[test]
fn parser_valid_includes() {
    let target_dir = TempDir::new("test").unwrap();

    JuteGenerator::new()
        .add_src_file("tests/parser/valid/includes.jute")
        .add_src_file("tests/parser/valid/types.jute")
        .add_out_dir(target_dir)
        .generate()
        .unwrap();
}

#[test]
fn parser_valid_containers() {
    let target_dir = TempDir::new("test").unwrap();

    JuteGenerator::new()
        .add_src_file("tests/parser/valid/containers.jute")
        .add_out_dir(target_dir)
        .generate()
        .unwrap();
}

#[test]
fn parser_invalid_syntax_error() {
    let target_dir = TempDir::new("test").unwrap();

    let res = JuteGenerator::new()
        .add_src_file("tests/parser/invalid/syntax_error.jute")
        .add_out_dir(target_dir)
        .generate();

    assert!(matches!(res, Err(JuteError::UnexpectedToken { .. })));
}

#[test]
fn parser_invalid_unknown_type() {
    let target_dir = TempDir::new("test").unwrap();

    let res = JuteGenerator::new()
        .add_src_file("tests/parser/invalid/unknown_type.jute")
        .add_out_dir(target_dir)
        .generate();
    assert!(matches!(res, Err(JuteError::UnknownType { .. })));
}

#[test]
fn parser_invalid_recursive_record() {
    let target_dir = TempDir::new("test").unwrap();

    let res = JuteGenerator::new()
        .add_src_file("tests/parser/invalid/recursive.jute")
        .add_out_dir(target_dir)
        .generate();

    assert!(matches!(res, Err(JuteError::CircularDependency { .. })));
}
