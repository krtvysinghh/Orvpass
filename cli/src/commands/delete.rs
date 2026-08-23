use dialoguer::{Confirm, Password};
use orvpass_core::crypto::SecretKey;
use orvpass_core::vault::Vault;
use std::path::PathBuf;

pub fn run(id: String) {
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
        Ok(_) => {
            if Confirm::new()
                .with_prompt("Delete this item?")
                .interact()
                .unwrap()
            {
                match vault.delete(id) {
                    Ok(_) => {
                        vault.save(&key).unwrap();

                        println!("✓ Deleted");
                    }

                    Err(e) => println!("Delete failed: {}", e),
                }
            }
        }

        Err(e) => println!("Unlock failed: {}", e),
    }
}

// audit hook: DELETE

#[allow(dead_code)]
fn audit_hook() {
    orvpass_core::security::audit::log("command");
}
