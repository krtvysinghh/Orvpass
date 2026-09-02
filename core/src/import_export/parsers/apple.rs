use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_apple_csv(csv_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    // Apple CSV header: Title,URL,Username,Password,Notes,OTPAuth
    for line in csv_str.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
        if parts.len() >= 4 {
            let title = parts[0];
            let url = parts[1];
            let username = parts[2];
            let password = parts[3];

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
