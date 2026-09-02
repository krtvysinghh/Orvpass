use crate::models::{ItemData, ItemType, LoginData, VaultItem};
use serde_json::Value;

pub fn parse_proton_json(json_str: &str) -> Result<Vec<VaultItem>, String> {
    let root: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    let mut items = Vec::new();

    if let Some(entries) = root.get("items").and_then(|v| v.as_array()) {
        for entry in entries {
            let title = entry.get("data").and_then(|d| d.get("title")).and_then(|v| v.as_str()).unwrap_or("Proton Item");
            let username = entry.get("data").and_then(|d| d.get("username")).and_then(|v| v.as_str()).map(|s| s.to_string());
            let password = entry.get("data").and_then(|d| d.get("password")).and_then(|v| v.as_str()).map(|s| s.to_string());

            let item = VaultItem::new(
                ItemType::Login,
                title,
                ItemData::Login(LoginData { username, password, urls: vec![] }),
            );
            items.push(item);
        }
    }
    Ok(items)
}
