use orvpass_core::{
    crypto::SecretKey,
    vault::{LockPolicy, Vault, VaultError, VaultState},
};

fn temp_vault_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("orvpass-lifecycle-{}.vault", uuid::Uuid::new_v4()))
}

#[test]
fn lifecycle_starts_locked() {
    let vault = Vault::new_locked();

    assert_eq!(vault.state(), VaultState::Locked);
    assert!(vault.is_locked());
    assert!(!vault.is_unlocked());
    assert_eq!(vault.failed_attempts(), 0);
}

#[test]
fn successful_unlock_resets_failure_counter() {
    let path = temp_vault_path();
    let key = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&key).unwrap();
    vault.lock();

    vault.unlock(&key).unwrap();

    assert_eq!(vault.state(), VaultState::Unlocked);
    assert_eq!(vault.failed_attempts(), 0);
    assert!(vault.unlocked_at().is_some());

    std::fs::remove_file(path).ok();
}

#[test]
fn wrong_password_increments_failure_counter() {
    let path = temp_vault_path();
    let correct = SecretKey::generate();
    let wrong = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&correct).unwrap();
    vault.lock();

    assert!(matches!(
        vault.unlock(&wrong),
        Err(VaultError::UnlockFailed)
    ));

    assert_eq!(vault.failed_attempts(), 1);
    assert_eq!(vault.state(), VaultState::Locked);

    std::fs::remove_file(path).ok();
}

#[test]
fn repeated_failures_trigger_temporary_lockout() {
    let path = temp_vault_path();
    let correct = SecretKey::generate();
    let wrong = SecretKey::generate();

    let policy = LockPolicy {
        max_failed_attempts: 3,
        lockout_seconds: 60,
    };

    let mut vault = Vault::new_locked_at(&path).with_policy(policy);
    vault.initialize(&correct).unwrap();
    vault.lock();

    assert!(matches!(
        vault.unlock(&wrong),
        Err(VaultError::UnlockFailed)
    ));

    assert!(matches!(
        vault.unlock(&wrong),
        Err(VaultError::UnlockFailed)
    ));

    assert!(matches!(
        vault.unlock(&wrong),
        Err(VaultError::UnlockFailed)
    ));

    assert_eq!(vault.state(), VaultState::TemporarilyLocked);

    assert!(matches!(
        vault.unlock(&correct),
        Err(VaultError::TemporarilyLocked)
    ));

    assert!(vault.remaining_lockout().is_some());

    std::fs::remove_file(path).ok();
}

#[test]
fn lock_clears_unlocked_state() {
    let path = temp_vault_path();
    let key = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&path);
    vault.initialize(&key).unwrap();

    assert_eq!(vault.state(), VaultState::Unlocked);

    vault.lock();

    assert_eq!(vault.state(), VaultState::Locked);
    assert!(vault.is_locked());
    assert!(!vault.is_unlocked());
    assert_eq!(vault.len(), 0);
    assert!(vault.unlocked_at().is_none());

    std::fs::remove_file(path).ok();
}
