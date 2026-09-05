use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_enpass_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(entries) = root.get("items").and_then(|v| v.as_array()) {
            for e in entries {
                let title = e.get("title").and_then(|v| v.as_str()).unwrap_or("Enpass Item");
                let mut username = None;
                let mut password = None;
                if let Some(fields) = e.get("fields").and_then(|v| v.as_array()) {
                    for f in fields {
                        let label = f.get("label").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
                        let val = f.get("value").and_then(|v| v.as_str()).map(|s| s.to_string());
                        if label.contains("user") || label.contains("email") {
                            username = val;
                        } else if label.contains("pass") {
                            password = val;
                        }
                    }
                }
                items.push(VaultItem::new(
                    ItemType::Login,
                    title,
                    ItemData::Login(LoginData { username, password, urls: vec![] }),
                ));
            }
        }
    }
    items
}
