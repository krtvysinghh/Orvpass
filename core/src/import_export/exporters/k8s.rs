use crate::models::{ItemData, VaultItem};

pub fn export_k8s_secret(items: &[VaultItem], name: &str) -> String {
    let mut entries = String::new();
    for item in items {
        if let ItemData::Login(l) = &item.data {
            let k = item.title.to_lowercase().replace([' ', '_', '.'], "-");
            let v = l.password.as_deref().unwrap_or("");
            entries.push_str(&format!("  {}: {}\n", k, v));
        }
    }
    format!(r#"apiVersion: v1
kind: Secret
metadata:
  name: {}
type: Opaque
stringData:
{}"#, name, entries)
}
