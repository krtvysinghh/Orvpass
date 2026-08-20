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

    let path = PathBuf::from(
        std::env::var("HOME")
            .unwrap()
    )
    .join(".orvpass");

    std::fs::create_dir_all(&path)
        .expect("cannot create vault directory");

    let mut vault = Vault::new();

    vault.initialize(&key)
        .expect("vault initialization failed");

    vault.save(&key)
        .expect("vault save failed");

    println!("Vault initialized at {:?}", path);
}
