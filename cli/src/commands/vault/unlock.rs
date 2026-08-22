use crate::vault::{encryption, session};

use std::io::{self, Write};

pub fn execute() {
    print!("Master password: ");

    io::stdout().flush().unwrap();

    let mut password = String::new();

    io::stdin().read_line(&mut password).unwrap();

    let password = password.trim();

    let salt = b"orvpass-master-salt";

    let key = encryption::derive_key(password, salt);

    session::unlock(orvpass_core::crypto::SecretKey(key));

    println!("Vault unlocked");
}
