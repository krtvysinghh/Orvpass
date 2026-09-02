use orvpass_core::import_export::exporters::html::export_standalone_html;
use orvpass_core::models::{ItemData, ItemType, LoginData, VaultItem};

#[test]
fn test_html_export_contains_structure() {
    let items = vec![VaultItem::new(
        ItemType::Login,
        "Test Service",
        ItemData::Login(LoginData {
            username: Some("user".into()),
            password: Some("pass".into()),
            urls: vec![],
        }),
    )];
    let html = export_standalone_html(&items);
    assert!(html.contains("Orvpass Emergency Vault"));
    assert!(html.contains("Test Service"));
}
