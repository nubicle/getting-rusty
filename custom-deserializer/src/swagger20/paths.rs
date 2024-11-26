use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Operation {
    pub id: String,
    pub path: Path,
}

/// The path of an API operation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct Path(pub String);

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
