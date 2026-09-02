use crate::models::{ItemData, ItemType, LoginData, SecureNoteData, VaultItem, CustomField};
use serde_json::Value;

pub fn parse_bitwarden_json(json_str: &str) -> Result<Vec<VaultItem>, String> {
    let root: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    let mut items = Vec::new();

    let entries = if let Some(arr) = root.get("items").and_then(|v| v.as_array()) {
        arr
    } else if let Some(arr) = root.as_array() {
        arr
    } else {
        return Ok(items);
    };

    for entry in entries {
        let name = entry.get("name").and_then(|v| v.as_str()).unwrap_or("Imported Item");
        let item_type = entry.get("type").and_then(|v| v.as_u64()).unwrap_or(1);
        let notes = entry.get("notes").and_then(|v| v.as_str()).unwrap_or("");

        let mut custom_fields = Vec::new();
        if let Some(fields) = entry.get("fields").and_then(|v| v.as_array()) {
            for f in fields {
                if let (Some(fname), Some(fval)) = (f.get("name").and_then(|v| v.as_str()), f.get("value").and_then(|v| v.as_str())) {
                    custom_fields.push(CustomField {
                        name: fname.to_string(),
                        value: fval.to_string(),
                        secret: f.get("type").and_then(|v| v.as_u64()).unwrap_or(0) == 1,
                    });
                }
            }
        }

        let item = match item_type {
            1 => {
                let login = entry.get("login");
                let username = login.and_then(|l| l.get("username")).and_then(|v| v.as_str()).map(|s| s.to_string());
                let password = login.and_then(|l| l.get("password")).and_then(|v| v.as_str()).map(|s| s.to_string());
                let mut urls = Vec::new();
                if let Some(uris) = login.and_then(|l| l.get("uris")).and_then(|v| v.as_array()) {
                    for u in uris {
                        if let Some(uri) = u.get("uri").and_then(|v| v.as_str()) {
                            urls.push(uri.to_string());
                        }
                    }
                }
                let mut v = VaultItem::new(ItemType::Login, name, ItemData::Login(LoginData { username, password, urls }));
                v.custom_fields = custom_fields;
                v
            }
            2 => {
                let mut v = VaultItem::new(ItemType::SecureNote, name, ItemData::SecureNote(SecureNoteData { content: notes.to_string() }));
                v.custom_fields = custom_fields;
                v
            }
            _ => {
                let mut v = VaultItem::new(ItemType::Custom("Other".into()), name, ItemData::Custom(std::collections::HashMap::new()));
                v.custom_fields = custom_fields;
                v
            }
        };
        items.push(item);
    }

    Ok(items)
}
