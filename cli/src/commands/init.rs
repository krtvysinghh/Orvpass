use anyhow::Result;
use dialoguer::Password;

pub fn run() {
    let password = Password::new()
        .with_prompt("Create master password")
        .interact()
        .unwrap();

    println!("Master password created ({} chars)", password.len());
}
