use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_keepass_csv(csv_str: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    for line in csv_str.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
        if parts.len() >= 5 {
            // KeePass CSV default columns: "Group","Title","Username","Password","URL","Notes"
            let title = parts[1];
            let username = parts[2];
            let password = parts[3];
            let url = parts[4];

            let item = VaultItem::new(
                ItemType::Login,
                title,
                ItemData::Login(LoginData {
                    username: Some(username.to_string()),
                    password: Some(password.to_string()),
                    urls: if url.is_empty() {
                        vec![]
                    } else {
                        vec![url.to_string()]
                    },
                }),
            );
            items.push(item);
        }
    }
    items
}
