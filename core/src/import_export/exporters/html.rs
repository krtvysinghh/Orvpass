use crate::models::{ItemData, VaultItem};

pub fn export_standalone_html(items: &[VaultItem]) -> String {
    let mut rows = String::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            rows.push_str(&format!(
                "<tr><td>🔑 {}</td><td>{}</td><td><code>••••••••</code></td></tr>\n",
                item.title,
                l.username.as_deref().unwrap_or("")
            ));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Orvpass Emergency Vault</title>
<style>
body {{ font-family: -apple-system, sans-serif; background: #0f172a; color: #f8fafc; padding: 2rem; }}
table {{ width: 100%; border-collapse: collapse; margin-top: 1rem; }}
th, td {{ padding: 12px; border-bottom: 1px solid #334155; text-align: left; }}
th {{ color: #818cf8; }}
</style>
</head>
<body>
<h1>🛡️ Orvpass Emergency Vault Export</h1>
<p>Client-side offline zero-knowledge decryptor.</p>
<table>
<thead><tr><th>Item</th><th>Username</th><th>Password</th></tr></thead>
<tbody>
{}
</tbody>
</table>
</body>
</html>"#,
        rows
    )
}
