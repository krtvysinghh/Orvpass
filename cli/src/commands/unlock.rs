use dialoguer::Password;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;
use std::path::PathBuf;

pub fn run() {
    let master = Password::new()
        .with_prompt("Master password")
        .interact()
        .unwrap();

    let key = SecretKey::from_password(&master).unwrap();

    let path = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".orvpass")
        .join("vault.orv");

    let mut vault = Vault::new(path);

    match vault.unlock(&key) {
        Ok(_) => println!("✓ Vault unlocked"),
        Err(e) => println!("✗ Unlock failed: {}", e),
    }
}
