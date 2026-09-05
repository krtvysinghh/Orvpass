use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_buttercup_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(entries) = root.get("entries").and_then(|v| v.as_array()) {
            for e in entries {
                let title = e.get("title").and_then(|v| v.as_str()).unwrap_or("Buttercup Entry");
                items.push(VaultItem::new(
                    ItemType::Login,
                    title,
                    ItemData::Login(LoginData { username: None, password: None, urls: vec![] }),
                ));
            }
        }
    }
    items
}
