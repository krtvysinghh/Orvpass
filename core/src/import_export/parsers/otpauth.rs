use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_otpauth_uris(content: &str) -> Vec<VaultItem> {
    let mut items = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("otpauth://") {
            let label = trimmed.split('?').next().unwrap_or("TOTP Account").replace("otpauth://totp/", "");
            items.push(VaultItem::new(
                ItemType::Totp,
                &label,
                ItemData::Login(LoginData { username: Some(label.clone()), password: None, urls: vec![trimmed.to_string()] }),
            ));
        }
    }
    items
}
