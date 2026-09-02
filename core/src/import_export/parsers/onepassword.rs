use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_1password_csv(csv_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    for line in csv_str.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
        if parts.len() >= 3 {
            let title = parts[0];
            let url = if parts.len() > 1 { parts[1] } else { "" };
            let username = if parts.len() > 2 { parts[2] } else { "" };
            let password = if parts.len() > 3 { parts[3] } else { "" };
            
            let item = VaultItem::new(
                ItemType::Login,
                title,
                ItemData::Login(LoginData {
                    username: Some(username.to_string()),
                    password: Some(password.to_string()),
                    urls: if url.is_empty() { vec![] } else { vec![url.to_string()] },
                }),
            );
            items.push(item);
        }
    }
    items
}
