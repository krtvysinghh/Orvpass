use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_lastpass_csv(csv_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    // LastPass CSV columns: url,username,password,totp,extra,name,grouping,fav
    for line in csv_str.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
        if parts.len() >= 3 {
            let url = parts[0];
            let username = parts[1];
            let password = parts[2];
            let title = if parts.len() > 5 && !parts[5].is_empty() { parts[5] } else { url };
            
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
