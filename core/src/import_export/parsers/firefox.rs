use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_firefox_csv(csv_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    for line in csv_str.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
        if parts.len() >= 3 {
            let url = parts[0];
            let username = parts[1];
            let password = parts[2];
            items.push(VaultItem::new(
                ItemType::Login,
                if url.is_empty() { "Firefox Login" } else { url },
                ItemData::Login(LoginData {
                    username: Some(username.to_string()),
                    password: Some(password.to_string()),
                    urls: if url.is_empty() { vec![] } else { vec![url.to_string()] },
                }),
            ));
        }
    }
    items
}
