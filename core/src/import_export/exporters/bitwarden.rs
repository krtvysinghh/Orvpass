use crate::models::{ItemData, VaultItem};

pub fn export_bitwarden_json(items: &[VaultItem]) -> String {
    let mut bw_items = Vec::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            bw_items.push(serde_json::json!({
                "name": item.title,
                "type": 1,
                "login": {
                    "username": l.username,
                    "password": l.password,
                    "uris": l.urls.iter().map(|u| serde_json::json!({"uri": u})).collect::<Vec<_>>()
                }
            }));
        }
    }
    serde_json::to_string_pretty(&serde_json::json!({
        "encrypted": false,
        "items": bw_items
    })).unwrap_or_default()
}
