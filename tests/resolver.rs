use jute::{JuteGenerator, errors::JuteError};
use tempdir::TempDir;

#[test]
fn resolver_valid_dependency_graph() {
    let target_dir = TempDir::new("test").unwrap();

    JuteGenerator::new()
        .add_src_file("tests/resolver/valid/dependency.jute")
        .add_out_dir(target_dir)
        .generate()
        .unwrap();
}

#[test]
fn resolver_invalid_cycle() {
    let target_dir = TempDir::new("test").unwrap();

    let res = JuteGenerator::new()
        .add_src_file("tests/resolver/invalid/cycle.jute")
        .add_out_dir(target_dir)
        .generate();

    assert!(matches!(res, Err(JuteError::CircularDependency { .. })));
}
