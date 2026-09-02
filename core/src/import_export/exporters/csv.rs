use crate::models::{ItemData, VaultItem};

pub fn export_standard_csv(items: &[VaultItem]) -> String {
    let mut out = String::from("title,username,password,urls,notes\n");
    for item in items {
        if let ItemData::Login(l) = &item.data {
            let u = l.username.as_deref().unwrap_or("");
            let p = l.password.as_deref().unwrap_or("");
            let url = l.urls.first().map(|s| s.as_str()).unwrap_or("");
            out.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"\"\n",
                item.title.replace('"', "\"\""),
                u.replace('"', "\"\""),
                p.replace('"', "\"\""),
                url.replace('"', "\"\"")
            ));
        }
    }
    out
}
