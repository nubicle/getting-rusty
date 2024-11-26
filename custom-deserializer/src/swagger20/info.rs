use serde::Deserialize;

/// The `"info"` property of an OpenAPI spec.
#[derive(Debug, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
}
