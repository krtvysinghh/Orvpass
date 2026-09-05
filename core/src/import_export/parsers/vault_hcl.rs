use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_hashicorp_vault_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(data) = root.get("data").and_then(|v| v.get("data").or(Some(v))).and_then(|v| v.as_object()) {
            for (k, v) in data {
                let val_str = v.as_str().map(|s| s.to_string()).unwrap_or_default();
                items.push(VaultItem::new(
                    ItemType::Login,
                    k,
                    ItemData::Login(LoginData { username: Some(k.clone()), password: Some(val_str), urls: vec![] }),
                ));
            }
        }
    }
    items
}
