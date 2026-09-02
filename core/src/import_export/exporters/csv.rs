use crate::models::{ItemData, VaultItem};

pub fn export_standard_csv(items: &[VaultItem]) -> String {
    let mut out = String::from("title,username,password,urls,notes\n");
    for item in items {
        if let ItemData::Login(l) = &item.data {
            out.push_str(&format!(
                "\"\",\"\",\"\",\"\",\"\"\n",
            ));
        }
    }
    out
}
