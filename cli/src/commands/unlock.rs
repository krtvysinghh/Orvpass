use dialoguer::Password;
use orvpass_core::crypto::SecretKey;

use crate::vault;
use crate::vault::session;

pub fn execute() {
    let master = Password::new()
        .with_prompt("Master password")
        .interact()
        .unwrap();

    let key = SecretKey::from_password(&master).unwrap();

    let mut v = vault::open();

    match v.unlock(&key) {
        Ok(_) => {
            session::set(key);
            println!("✓ Vault unlocked");
        }
        Err(e) => {
            println!("✗ Unlock failed: {}", e);
        }
    }
}
