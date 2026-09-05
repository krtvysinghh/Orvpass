use crate::models::{ItemData, VaultItem};

pub fn export_vault_kv_json(items: &[VaultItem]) -> String {
    let mut map = serde_json::Map::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            map.insert(item.title.clone(), serde_json::Value::String(l.password.clone().unwrap_or_default()));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({
        "data": map
    })).unwrap_or_default()
}
