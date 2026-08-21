use dialoguer::Password;
use orvpass_core::crypto::SecretKey;
use orvpass_core::models::{ItemData, ItemType, LoginData, VaultItem};
use orvpass_core::vault::Vault;
use std::path::PathBuf;

pub fn run() {
    let title = dialoguer::Input::<String>::new()
        .with_prompt("Title")
        .interact_text()
        .unwrap();

    let username = dialoguer::Input::<String>::new()
        .with_prompt("Username")
        .interact_text()
        .unwrap();

    let password = dialoguer::Password::new()
        .with_prompt("Password")
        .interact()
        .unwrap();

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

    vault
        .insert(VaultItem::new(
            ItemType::Login,
            &title,
            ItemData::Login(LoginData {
                username: Some(username),
                password: Some(password),
                urls: vec![],
            }),
        ))
        .unwrap();

    vault.save(&key).unwrap();

    println!("✓ Credential saved");
}

// audit hook: ADD

#[allow(dead_code)]
fn audit_hook() {
    orvpass_core::security::audit::log("command");
}
