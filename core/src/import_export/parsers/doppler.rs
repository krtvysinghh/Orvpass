use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_doppler_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(obj) = root.as_object() {
            for (key, val) in obj {
                let secret = val.as_str().or_else(|| val.get("computed").and_then(|v| v.as_str())).map(|s| s.to_string());
                items.push(VaultItem::new(
                    ItemType::Login,
                    key,
                    ItemData::Login(LoginData { username: Some(key.clone()), password: secret, urls: vec![] }),
                ));
            }
        }
    }
    items
}
