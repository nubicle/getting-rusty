use serde::Deserialize;

/// The path of an API operation.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Path(pub String);
