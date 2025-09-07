// Auto-generated from Swagger
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub roles: Option<Vec<String>>,
    pub isActive: bool,
    pub address: Option<serde_json::Value>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub postalCode: String,
    pub street: String,
    pub city: String,
    pub country: String,
}
