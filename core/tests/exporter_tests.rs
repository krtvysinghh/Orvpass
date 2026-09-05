use orvpass_core::models::{VaultItem, ItemType, ItemData, LoginData};
use orvpass_core::import_export::exporters::dotenv::export_dotenv;
use orvpass_core::import_export::exporters::k8s::export_k8s_secret;

#[test]
fn test_dotenv_export() {
    let items = vec![VaultItem::new(ItemType::Login, "APP_SECRET", ItemData::Login(LoginData {
        username: None, password: Some("supersecret".into()), urls: vec![]
    }))];
    let res = export_dotenv(&items);
    assert!(res.contains("APP_SECRET=supersecret"));
}

#[test]
fn test_k8s_export() {
    let items = vec![VaultItem::new(ItemType::Login, "api-key", ItemData::Login(LoginData {
        username: None, password: Some("k8spass".into()), urls: vec![]
    }))];
    let res = export_k8s_secret(&items, "my-secret");
    assert!(res.contains("name: my-secret"));
}
