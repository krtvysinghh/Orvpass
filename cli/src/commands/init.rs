use dialoguer::Password;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;
use std::path::PathBuf;

pub fn run() {
    let password = Password::new()
        .with_prompt("Create master password")
        .interact()
        .unwrap();

    let key = SecretKey::from_password(&password).unwrap();

    let dir = PathBuf::from(std::env::var("HOME").unwrap()).join(".orvpass");

    std::fs::create_dir_all(&dir).unwrap();

    let mut vault = Vault::new(dir.join("vault.orv"));

    vault.initialize(&key).unwrap();

    println!("Vault initialized");
}
