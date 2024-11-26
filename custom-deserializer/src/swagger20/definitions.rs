use serde::Deserialize;

/// A definition path.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct DefinitionPath(pub String);
