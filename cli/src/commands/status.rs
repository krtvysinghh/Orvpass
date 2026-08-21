use std::path::PathBuf;

pub fn run() {
    let base = PathBuf::from(std::env::var("HOME").unwrap()).join(".orvpass");

    println!("Orvpass Status");

    println!("Directory: {}", base.exists());

    println!("Vault: {}", base.join("vault.orv").exists());
}
