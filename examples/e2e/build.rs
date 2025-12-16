use jute::JuteGenerator;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("generated");
    // Clean previous output
    let x = fs::remove_dir_all("./src/generated");
    println!("{:?}", x);
    // 1️⃣ Generate Rust code from Jute
    JuteGenerator::new()
        .add_src_file("./schema/model.jute")
        .add_src_file("./schema/common.jute")
        .add_out_dir(&out_dir)
        .generate()
        .unwrap();
    println!("cargo:rerun-if-changed=schema/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=generated/");
}
