use std::{collections::HashMap, path::Path};

use domain::enums::*;
use domain::models::*;
use schemars::{schema::RootSchema, schema_for};

fn write_schema(dir: &std::path::Path, name: &str, schema: &RootSchema) -> std::io::Result<()> {
    let output = serde_json::to_string_pretty(schema).unwrap();
    let output_path = dir.join(format!("{}.json", name));
    std::fs::write(output_path, output)
}

fn main() {
    let dir = Path::new("./src/schemas");

    let data_types = HashMap::from([
        ("review", schema_for!(review::Review)),
        ("new_review", schema_for!(review::NewReview)),
        ("unit", schema_for!(unit::Unit)),
        ("user", schema_for!(user::User)),
        ("new_user", schema_for!(user::NewUser)),
        ("semester", schema_for!(semester::Semester)),
    ]);

    for (file_name, data_type) in data_types {
        if let Err(err) = write_schema(dir, file_name, &data_type) {
            panic!("Error creating schema for type {} - {}", file_name, err);
        };
    };
}
