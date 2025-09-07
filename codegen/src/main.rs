use std::{collections::HashMap, fs};

mod types;
use serde::Deserialize;

fn main() -> std::io::Result<()> {
    const FILENAME: &str = "swagger.json";
    let spec: Spec = serde_json::from_str(&fs::read_to_string(FILENAME)?)?;
    let mut rust_code =
        String::from("// Auto-generated from Swagger\nuse serde::{Serialize, Deserialize};\n\n");

    for (name, def) in &spec.definitions {
        rust_code.push_str(&generate_struct(name, def));
        rust_code.push_str("\n");
    }

    std::fs::write("src/types.rs", rust_code)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub info: HashMap<String, String>,
    // pub paths: HashMap<String, serde_json::Value>,
    pub definitions: HashMap<String, Definition>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    #[serde(rename = "type")]
    pub def_type: Option<String>,
    pub properties: Option<HashMap<String, Property>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub prop_type: Option<String>,
    pub format: Option<String>,
    pub items: Option<Box<Property>>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
}

fn swagger_type_to_rust_type(prop: &Property) -> String {
    match prop.prop_type.as_deref() {
        Some("boolean") => "bool".into(),
        Some("string") => "String".into(),
        Some("integer") => match prop.format.as_deref() {
            Some("int64") => "i64".into(),
            _ => "i32".into(),
        },
        Some("array") => {
            let item_type = prop
                .items
                .as_ref()
                .map_or("serde_json::Value".into(), |item| {
                    swagger_type_to_rust_type(item)
                });
            format!("Vec<{}>", item_type)
        }
        Some("object") => {
            if let Some(reference) = &prop.reference {
                // extract type name from "#/definitions/Address"
                let parts: Vec<_> = reference.split('/').collect();
                parts.last().unwrap_or(&"Unknown").to_string()
            } else {
                "serde_json::Value".into()
            }
        }
        _ => "serde_json::Value".into(),
    }
}

fn generate_struct(name: &str, schema: &Definition) -> String {
    let mut output = format!(
        "#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{\n",
        name
    );

    if let Some(props) = &schema.properties {
        let empty_vec: Vec<String> = vec![];
        let required = schema.required.as_ref().unwrap_or(&empty_vec);

        for (field_name, prop) in props {
            let rust_type = swagger_type_to_rust_type(prop);
            let optional = !required.contains(field_name);
            let field = if optional {
                format!("  pub {}: Option<{}>,\n", field_name, rust_type)
            } else {
                format!("  pub {}: {},\n", field_name, rust_type)
            };
            output.push_str(&field);
        }
    }

    output.push_str("}\n");
    output
}
