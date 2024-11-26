use serde::Deserialize;
use std::collections::BTreeMap;

mod definitions;
use definitions::*;
mod info;
use info::*;
mod paths;
use paths::*;

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub info: Info,
    pub definitions: BTreeMap<DefinitionPath, serde_json::Value>,
    pub paths: BTreeMap<Path, serde_json::Value>,
}
