use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_k8s_secret_json(json_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    if let Ok(root) = serde_json::from_str::<Value>(json_str) {
        let name = root.get("metadata").and_then(|m| m.get("name")).and_then(|v| v.as_str()).unwrap_or("k8s-secret");
        if let Some(data) = root.get("data").and_then(|v| v.as_object()) {
            for (k, _v) in data {
                items.push(VaultItem::new(
                    ItemType::Login,
                    &format!("{}/{}", name, k),
                    ItemData::Login(LoginData { username: Some(k.clone()), password: None, urls: vec![] }),
                ));
            }
        }
    }
    items
}
