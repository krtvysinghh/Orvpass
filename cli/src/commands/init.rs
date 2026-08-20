use dialoguer::Password;
use orvpass_core::vault::Vault;
use orvpass_core::crypto::SecretKey;

pub fn run() {
    let password = Password::new()
        .with_prompt("Create master password")
        .interact()
        .unwrap();

    let key = SecretKey::from_password(password.as_bytes()).unwrap();

    let mut vault = Vault::new();

    vault.initialize(&key).unwrap();

    println!("Vault initialized");
}
