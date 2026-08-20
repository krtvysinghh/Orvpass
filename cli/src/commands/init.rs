use dialoguer::Password;
use std::path::PathBuf;
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;

pub fn run() {
    let password = Password::new()
        .with_prompt("Create master password")
        .interact()
        .unwrap();

    let key = SecretKey::from_password(password.as_bytes())
        .expect("key derivation failed");

    let vault_dir = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".orvpass");

    std::fs::create_dir_all(&vault_dir)
        .expect("vault directory failed");

    let vault_file = vault_dir.join("vault.orv");

    let mut vault = Vault::new();

    vault.initialize(&key)
        .expect("vault init failed");

    vault.save(&key)
        .expect("vault encryption failed");

    std::fs::write(
        vault_file,
        b"encrypted vault created"
    )
    .expect("vault file failed");

    println!("Vault initialized");
}
