use orvpass_core::{
    crypto::SecretKey,
    models::{ItemData, ItemType, SecureNoteData, VaultItem},
    vault::{Vault, VaultError},
};

fn temp_vault_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("orvpass-test-{}.vault", uuid::Uuid::new_v4()))
}

#[test]
fn new_vault_starts_locked() {
    let vault = Vault::new_locked();

    assert!(vault.is_locked());
    assert!(vault.is_empty());
}

#[test]
fn locked_vault_rejects_mutation() {
    let mut vault = Vault::new_locked();

    let result = vault.create_login("GitHub");

    assert!(matches!(result, Err(VaultError::Locked)));
}

#[test]
fn vault_can_create_login_after_unlock() {
    let path = temp_vault_path();
    let key = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&key).unwrap();

    let id = vault.create_login("GitHub").unwrap();

    assert_eq!(vault.len(), 1);
    assert!(vault.item(id).unwrap().is_some());

    std::fs::remove_file(path).ok();
}

#[test]
fn vault_persists_encrypted_data() {
    let path = temp_vault_path();
    let key = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&key).unwrap();

    let item = VaultItem::new(
        ItemType::SecureNote,
        "Private",
        ItemData::SecureNote(SecureNoteData {
            content: "classified".into(),
        }),
    );

    let id = item.id;

    vault.insert(item).unwrap();
    vault.save(&key).unwrap();
    vault.lock();

    assert!(vault.is_locked());

    vault.unlock(&key).unwrap();

    assert!(!vault.is_locked());
    assert_eq!(vault.len(), 1);
    assert!(vault.item(id).unwrap().is_some());

    std::fs::remove_file(path).ok();
}

#[test]
fn wrong_key_cannot_unlock_vault() {
    let path = temp_vault_path();
    let correct = SecretKey::generate();
    let wrong = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&correct).unwrap();
    vault.save(&correct).unwrap();
    vault.lock();

    assert!(vault.unlock(&wrong).is_err());

    std::fs::remove_file(path).ok();
}

#[test]
fn vault_file_is_not_plaintext() {
    let path = temp_vault_path();
    let key = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&key).unwrap();

    let item = VaultItem::new(
        ItemType::SecureNote,
        "Sensitive",
        ItemData::SecureNote(SecureNoteData {
            content: "VERY-SECRET-ORVPASS-DATA".into(),
        }),
    );

    vault.insert(item).unwrap();
    vault.save(&key).unwrap();

    let raw = std::fs::read(&path).unwrap();

    assert!(
        !raw.windows(b"VERY-SECRET-ORVPASS-DATA".len())
            .any(|window| window == b"VERY-SECRET-ORVPASS-DATA")
    );

    std::fs::remove_file(path).ok();
}

#[test]
fn empty_vault_reports_empty() {
    let vault = Vault::new_locked();

    assert!(vault.is_empty());
    assert_eq!(vault.len(), 0);
}
