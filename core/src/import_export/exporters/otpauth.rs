use crate::models::{ItemData, VaultItem};

pub fn export_otpauth_uris(items: &[VaultItem]) -> String {
    let mut out = String::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            let secret = l.password.as_deref().unwrap_or("TESTSECRET");
            out.push_str(&format!("otpauth://totp/Orvpass:{}?secret={}&issuer=Orvpass\n", item.title, secret));
        }
    }
    out
}
