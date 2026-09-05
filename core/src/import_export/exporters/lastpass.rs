use crate::models::{ItemData, VaultItem};

pub fn export_lastpass_csv(items: &[VaultItem]) -> String {
    let mut out = String::from("url,username,password,totp,extra,name,grouping,fav\n");
    for item in items {
        if let ItemData::Login(l) = &item.data {
            let u = l.urls.first().map(|s| s.as_str()).unwrap_or("");
            out.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"\",\"\",\"{}\",\"\",0\n",
                u.replace('"', "\"\""),
                l.username.as_deref().unwrap_or("").replace('"', "\"\""),
                l.password.as_deref().unwrap_or("").replace('"', "\"\""),
                item.title.replace('"', "\"\"")
            ));
        }
    }
    out
}
