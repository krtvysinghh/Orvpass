use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultItem {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

impl VaultItem {
    pub fn new(
        id: String,
        name: String,
        username: String,
        password: String,
        url: String,
    ) -> Self {
        Self {
            id,
            name,
            username,
            password,
            url,
        }
    }
}
