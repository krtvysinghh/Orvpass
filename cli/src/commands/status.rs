use crate::vault::database;

pub fn execute() {
    let items = database::load_items();
    let path = database::vault_path();
    let exists = path.exists();

    println!("🛡️  ORVPASS VAULT STATUS");
    println!("========================");
    println!("  Vault Location: {}", path.display());
    println!("  Vault Encrypted: {}", if exists { "Yes (Argon2id + ChaCha20-Poly1305)" } else { "Not Initialized (Default In-Memory)" });
    println!("  Total Items:    {}", items.len());
    println!("  Security State: 🔒 Zero-Knowledge RAM Encrypted");
    println!("========================");
}
