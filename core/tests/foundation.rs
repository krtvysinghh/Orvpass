use orvpass_core::{
    info,
    models::{ItemType, VaultItem},
    totp::{TotpAlgorithm, TotpConfig},
    vault::Vault,
};

#[test]
fn product_identity() {
    let (name, version) = info();
    assert_eq!(name, "Orvpass");
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn vault_starts_locked() {
    let vault = Vault::new_locked();
    assert!(vault.is_locked());
    assert_eq!(vault.len(), 0);
}

#[test]
fn items_have_unique_ids() {
    let a = VaultItem::new(ItemType::Login, "GitHub");
    let b = VaultItem::new(ItemType::Login, "GitHub");
    assert_ne!(a.id, b.id);
}

#[test]
fn totp_defaults() {
    let config = TotpConfig::default();
    assert_eq!(config.digits, 6);
    assert_eq!(config.period_seconds, 30);
    assert_eq!(config.algorithm, TotpAlgorithm::Sha1);
}
