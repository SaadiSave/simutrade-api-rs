use serde::{Deserialize, Serialize};

// representation of user in storage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}
