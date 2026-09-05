use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_dotenv_content(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim();
                let val = v.trim().trim_matches('"').trim_matches('\'');
                items.push(VaultItem::new(
                    ItemType::Login,
                    key,
                    ItemData::Login(LoginData { username: Some(key.to_string()), password: Some(val.to_string()), urls: vec![] }),
                ));
            }
        }
    }
    items
}
