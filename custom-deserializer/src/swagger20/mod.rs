use std::collections::BTreeMap;

mod definitions;
use definitions::*;
mod info;
use info::*;
mod paths;
use paths::*;

#[derive(Debug)]
pub struct Spec {
    pub info: Info,
    pub definitions: BTreeMap<DefinitionPath, serde_json::Value>,
    pub operations: Vec<Operation>,
}

impl<'de> serde::Deserialize<'de> for Spec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, serde::Deserialize)]
        pub struct InnerSpec {
            info: Info,
            definitions: BTreeMap<DefinitionPath, serde_json::Value>,
            paths: BTreeMap<Path, InnerPathItem>,
            swagger: String,
        }

        #[derive(Debug, serde::Deserialize)]
        pub struct InnerPathItem {
            delete: Option<InnerOperation>,
            get: Option<InnerOperation>,
        }

        #[derive(Debug, serde::Deserialize)]
        pub struct InnerOperation {
            #[serde(rename = "operationId")]
            id: String,
        }

        let result: InnerSpec = serde::Deserialize::deserialize(deserializer)?;

        if result.swagger != "2.0" {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&result.swagger),
                &"2.0",
            ));
        }

        let mut operations = vec![];

        for (path, path_item) in result.paths {
            if let Some(delete) = path_item.delete {
                operations.push(Operation {
                    id: delete.id,
                    path: path.clone(),
                })
            }

            if let Some(get) = path_item.get {
                operations.push(Operation {
                    id: get.id,
                    path: path.clone(),
                })
            }
        }

        Ok(Spec {
            info: result.info,
            definitions: result.definitions,
            operations,
        })
    }
}
