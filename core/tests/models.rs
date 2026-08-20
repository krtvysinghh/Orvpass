use orvpass_core::models::{Folder, ItemData, ItemType, LoginData, SecureNoteData, VaultItem};

#[test]
fn login_item_has_expected_structure() {
    let item = VaultItem::new(
        ItemType::Login,
        "GitHub",
        ItemData::Login(LoginData {
            username: Some("kartavya".into()),
            password: Some("example".into()),
            urls: vec!["https://github.com".into()],
        }),
    );

    assert_eq!(item.title, "GitHub");
    assert_eq!(item.item_type, ItemType::Login);
    assert!(!item.id.is_nil());
}

#[test]
fn secure_note_is_supported() {
    let item = VaultItem::new(
        ItemType::SecureNote,
        "Private Note",
        ItemData::SecureNote(SecureNoteData {
            content: "secret".into(),
        }),
    );

    assert_eq!(item.item_type, ItemType::SecureNote);
}

#[test]
fn tags_do_not_duplicate() {
    let mut item = VaultItem::new(ItemType::Login, "GitHub", ItemData::Custom);

    item.add_tag("development");
    item.add_tag("development");

    assert_eq!(item.tags.len(), 1);
}

#[test]
fn custom_secret_fields_work() {
    let mut item = VaultItem::new(ItemType::Custom, "API", ItemData::Custom);

    item.add_custom_field("API Secret", "secret-value", true);

    assert_eq!(item.custom_fields.len(), 1);
    assert!(item.custom_fields[0].secret);
}

#[test]
fn folder_has_unique_id() {
    let first = Folder::new("Personal");
    let second = Folder::new("Personal");

    assert_ne!(first.id, second.id);
}
