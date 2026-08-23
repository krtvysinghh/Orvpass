use dialoguer::Password;
use orvpass_core::crypto::SecretKey;

use crate::vault;
use crate::vault::session;

pub fn execute() {
    let password = Password::new()
        .with_prompt("Create master password")
        .interact()
        .unwrap();

    let key = SecretKey::from_password(&password).unwrap();

    let dir = std::env::var("HOME").unwrap();

    std::fs::create_dir_all(format!("{}/.orvpass", dir)).unwrap();

    let mut v = vault::open();

    v.initialize(&key).unwrap();

    session::set(key);

    println!("✓ Vault initialized");
}
