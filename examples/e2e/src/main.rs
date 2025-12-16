use std::{fs, path::PathBuf};

use jute::JuteGenerator;

// must import this create to use serialize and deserialize methods
use jute::code_generator::rust::JuteSerializable;

// Bring generated code into scope please make sure you do this for all the root modules (inside generated code)
pub mod serialization {
    include!("./generated/serialization/mod.rs");
}
pub mod common {
    include!("./generated/common/mod.rs");
}

use common::Address;
use serialization::{Company, Person};

fn main() -> Result<(), jute::errors::JuteError> {
    let owner = Person {
        id: 1,
        name: "Alice".into(),
        active: true,
        address: Address {
            city: "Berlin".into(),
            country: "DE".into(),
        },
    };

    let employee = Person {
        id: 2,
        name: "Bob".into(),
        active: false,
        address: Address {
            city: "Paris".into(),
            country: "FR".into(),
        },
    };

    let mut company = Company {
        id: 42,
        owner,
        employees: vec![employee.clone()],
        directory: std::collections::HashMap::new(),
    };

    company.directory.insert(employee.name.clone(), employee);

    let mut buffer = Vec::new();
    company.serialize(&mut buffer)?;

    let decoded = Company::deserialize(&mut &buffer[..])?;
    assert_eq!(company, decoded);

    println!("✅ Serialization round-trip successful!");

    Ok(())
}
