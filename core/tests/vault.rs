use orvpass_core::vault::Vault;

#[test]
fn vault_can_create_login() {
    let mut vault = Vault::new_locked();

    let id = vault.create_login("GitHub");

    assert_eq!(vault.len(), 1);
    assert!(vault.item(id).is_some());
}

#[test]
fn empty_vault_reports_empty() {
    let vault = Vault::new_locked();

    assert!(vault.is_empty());
    assert_eq!(vault.len(), 0);
}
