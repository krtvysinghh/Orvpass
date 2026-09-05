use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_infisical_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        if let Some(secrets) = root.get("secrets").and_then(|v| v.as_array()) {
            for s in secrets {
                let key = s.get("secretKey").and_then(|v| v.as_str()).unwrap_or("INFISICAL_KEY");
                let val = s.get("secretValue").and_then(|v| v.as_str()).map(|s| s.to_string());
                items.push(VaultItem::new(
                    ItemType::Login,
                    key,
                    ItemData::Login(LoginData { username: Some(key.to_string()), password: val, urls: vec![] }),
                ));
            }
        }
    }
    items
}
