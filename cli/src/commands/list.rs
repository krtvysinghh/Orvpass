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

    vault.unlock(&key).unwrap();

    println!("Vault items: {}", vault.item_count());

    for item in vault.items() {
        println!("- {} [{:?}]", item.title, item.item_type);
    }
}
