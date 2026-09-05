use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_passbolt_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(entries) = root.as_array() {
            for e in entries {
                let title = e.get("name").and_then(|v| v.as_str()).unwrap_or("Passbolt Secret");
                let username = e.get("username").and_then(|v| v.as_str()).map(|s| s.to_string());
                items.push(VaultItem::new(
                    ItemType::Login,
                    title,
                    ItemData::Login(LoginData { username, password: None, urls: vec![] }),
                ));
            }
        }
    }
    items
}
