use dialoguer::Password;
use std::path::PathBuf;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;

pub fn run() {
    let _password = Password::new()
        .with_prompt("Master password")
        .interact()
        .unwrap();

    let vault_file = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".orvpass")
        .join("vault.orv");

    let key = SecretKey::generate();

    let mut vault = Vault::new_locked_at(&vault_file);

    vault.unlock(&key)
        .expect("vault unlock failed");

    println!("Vault unlocked");
}
