use jute::{JuteGenerator, errors::JuteError};
use tempdir::TempDir;

#[test]
fn includes_valid() {
    let target_dir = TempDir::new("test").unwrap();
    JuteGenerator::new()
        .add_src_file("tests/includes/valid/child.jute")
        .add_src_file("tests/includes/valid/base.jute")
        .add_out_dir(target_dir)
        .generate()
        .unwrap();
}

#[test]
fn includes_invalid_ambigous() {
    let target_dir = TempDir::new("test").unwrap();

    let res = JuteGenerator::new()
        .add_src_file("tests/includes/invalid/ambigous.jute")
        .add_src_file("tests/includes/invalid/mod1.jute")
        .add_src_file("tests/includes/invalid/mod2.jute")
        .add_out_dir(target_dir)
        .generate();
    assert!(matches!(res, Err(JuteError::AmbiguousType { .. })));
}

#[test]
fn include_invalid_missing_file() {
    let target_dir = TempDir::new("test").unwrap();
    let res = JuteGenerator::new()
        .add_src_file("tests/includes/invalid/missing_file.jute")
        .add_out_dir(target_dir)
        .generate();
    assert!(matches!(res, Err(JuteError::PathCanonicalizeError { .. })));
}
