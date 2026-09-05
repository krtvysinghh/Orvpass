use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_gcp_secrets_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(list) = root.get("secrets").and_then(|v| v.as_array()) {
            for s in list {
                let name = s.get("name").and_then(|v| v.as_str()).unwrap_or("GCP Secret");
                items.push(VaultItem::new(
                    ItemType::Login,
                    name,
                    ItemData::Login(LoginData { username: Some(name.to_string()), password: None, urls: vec![] }),
                ));
            }
        }
    }
    items
}
