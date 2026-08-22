use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultItem {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

impl VaultItem {
    pub fn new(
        name: String,
        username: String,
        password: String,
        url: String,
    ) -> Self {
        Self {
            name,
            username,
            password,
            url,
        }
    }
}
