use std::path::PathBuf;

use jute::JuteGenerator;

#[test]
fn generate_serialization_code() {
    let out_dir = PathBuf::from("generated");

    // Clean first (important)
    let _ = std::fs::remove_dir_all("src/generated");
    JuteGenerator::new()
        .add_src_file("tests/serialization/roundtrip.jute")
        .add_src_file("tests/serialization/pem.jute")
        .add_out_dir(&out_dir)
        .generate()
        .expect("code generation failed");
    // this line should be commented if you want to keep the generated code for serialization code
    let _ = std::fs::remove_dir_all("src/generated");
}

// uncomment this code only after generating code and keeping the generated files by commneting above test

// #[allow(dead_code)]
// pub mod serialization {
//     // Pull in generated code at compile time
//     include!("../src/generated/serialization/mod.rs");
// }

// use jute::code_generator::rust::JuteSerializable;
// use serialization::*;
// use std::collections::HashMap;

// #[test]
// fn complex_serialization_roundtrip() {
//     // --- Addresses ---
//     let addr_owner = pem::Address {
//         street: "221B Baker Street".into(),
//         city: "London".into(),
//         zip: 12345,
//     };

//     let addr_employee = pem::Address {
//         street: "742 Evergreen Terrace".into(),
//         city: "Springfield".into(),
//         zip: 54321,
//     };

//     // --- People ---
//     let owner = Person {
//         id: 1,
//         name: "Alice".into(),
//         active: true,
//         address: addr_owner,
//         phone_numbers: vec!["+44-1234-567890".into(), "+44-9876-543210".into()],
//     };

//     let employee = Person {
//         id: 2,
//         name: "Bob".into(),
//         active: false,
//         address: addr_employee,
//         phone_numbers: vec!["+1-555-0199".into()],
//     };

//     // --- Department metadata ---
//     let mut dept_meta = HashMap::new();
//     dept_meta.insert("floor".into(), "5".into());
//     dept_meta.insert("building".into(), "HQ-East".into());

//     // --- Department ---
//     let department = Department {
//         id: 101,
//         name: "Engineering".into(),
//         metadata: dept_meta,
//         members: vec![owner.clone(), employee.clone()],
//     };

//     // --- Primitives ---
//     let primitives = pem::Primitives {
//         flag: true,
//         small: 7,
//         count: 42,
//         total: 1_000_000,
//         ratio: 0.75,
//         precise: 3.1415926535,
//         name: "metrics".into(),
//         raw: vec![1, 2, 3, 4, 5],
//     };

//     // --- Company key people ---
//     let mut key_people = HashMap::new();
//     key_people.insert("owner".into(), owner.clone());

//     // --- Company ---
//     let company = Company {
//         id: 5001,
//         name: "Acme Corp".into(),
//         stats: primitives,
//         headquarters: owner.address.clone(),
//         departments: vec![department],
//         key_people,
//     };

//     // --- Organization index ---
//     let mut indexed_companies = HashMap::new();
//     indexed_companies.insert("acme".into(), company.clone());

//     // --- Organization (TOP LEVEL) ---
//     let org = Organization {
//         org_id: 9000,
//         org_name: "Global Holdings".into(),
//         companies: vec![company],
//         indexed_companies,
//     };

//     // ================= SERIALIZE =================
//     let mut buffer = Vec::new();
//     org.serialize(&mut buffer).expect("serialization failed");

//     // ================= DESERIALIZE ===============
//     let decoded = Organization::deserialize(&mut &buffer[..]).expect("deserialization failed");

//     // ================= VERIFY ====================
//     assert_eq!(org, decoded);
// }
