use std::{collections::HashMap, fs};

use serde::Deserialize;

fn main() {
    let Ok(content) = fs::read("swagger.json") else {
        eprintln!("failed to read file: swagger.json");

        std::process::exit(1)
    };

    let Ok(spec) = serde_json::from_slice::<Spec>(&content) else {
        eprintln!("failed to deserialize from content");

        std::process::exit(1)
    };

    println!("{:?}", spec.definitions);
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub info: HashMap<String, String>,
    pub paths: HashMap<String, serde_json::Value>,
    pub definitions: HashMap<String, serde_json::Value>,
}
