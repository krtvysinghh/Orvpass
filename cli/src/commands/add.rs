use crate::vault;
use crate::vault::session;

use dialoguer::Password;
use orvpass_core::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn execute(name: String) {
    let Some(key) = session::key() else {
        println!("Vault locked");
        return;
    };

    let mut vault = vault::open();

    if vault.unlock(&key).is_err() {
        println!("Unlock failed");
        return;
    }

    let password = Password::new()
        .with_prompt("Password")
        .interact()
        .unwrap();

    let item = VaultItem::new(
        ItemType::Login,
        &name,
        ItemData::Login(LoginData {
            username: None,
            password: Some(password),
            urls: vec![],
        }),
    );

    if vault.insert(item).is_ok() {
        let _ = vault.save(&key);
        println!("Added {}", name);
    }
}
