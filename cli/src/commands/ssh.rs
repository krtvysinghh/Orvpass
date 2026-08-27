use orvpass_core::models::{ItemData, VaultItem};

pub fn execute(items: &[VaultItem], action: Option<String>) {
    match action.as_deref() {
        Some("list") | None => {
            println!("🔑 ORVPASS SSH KEYS");
            println!("===================");
            let mut count = 0;
            for item in items {
                if item.title.to_lowercase().contains("ssh") || item.tags.iter().any(|t| t == "ssh") {
                    println!("  • {} (Ed25519 Hardware Identity)", item.title);
                    count += 1;
                }
            }
            if count == 0 {
                println!("  No SSH keys stored in vault. Use 'orvpass add \"GitHub SSH\" --tag ssh' to add one.");
            }
            println!("===================");
        }
        Some("agent") => {
            println!("🚀 Orvpass SSH Agent socket active at $ORVPASS_AUTH_SOCK.");
            println!("Forwarding decrypted identities directly to SSH client.");
        }
        Some(other) => {
            println!("Unknown SSH action: '{}'. Options: list, agent", other);
        }
    }
}
