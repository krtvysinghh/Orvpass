use crate::models::{ItemData, VaultItem};

pub fn export_dotenv(items: &[VaultItem]) -> String {
    let mut out = String::from("# Exported by Orvpass Enterprise\n");
    for item in items {
        if let ItemData::Login(l) = &item.data {
            let key = item.title.to_uppercase().replace([' ', '-', '.'], "_");
            let val = l.password.as_deref().unwrap_or("");
            out.push_str(&format!("{}={}\n", key, val));
        }
    }
    out
}
