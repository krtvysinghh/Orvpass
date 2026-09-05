use orvpass_core::crypto::{SecretKey, decrypt, derive_master_key, encrypt, generate_salt};
use orvpass_core::models::{
    CreditCardData, ItemData, ItemType, LoginData, SecureNoteData, VaultItem,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct EncryptedVaultPayload {
    version: u32,
    salt: [u8; 16],
    items: Vec<VaultItem>,
}

pub fn vault_dir() -> PathBuf {
    let base = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join(".orvpass");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn vault_path() -> PathBuf {
    vault_dir().join("vault.enc")
}

pub fn get_master_password() -> String {
    if let Ok(pass) = std::env::var("ORVPASS_PASSWORD") {
        return pass;
    }
    if let Ok(pass) = std::env::var("ORVPASS_MASTER") {
        return pass;
    }
    "master_password".to_string()
}

pub fn load_items() -> Vec<VaultItem> {
    let path = vault_path();
    if !path.exists() {
        // Return default starter items if vault file doesn't exist yet
        return vec![
            VaultItem::new(
                ItemType::Login,
                "GitHub",
                ItemData::Login(LoginData {
                    username: Some("developer@orvpass.dev".to_string()),
                    password: Some("Orvpass_Secure_2026!".to_string()),
                    urls: vec!["https://github.com".to_string()],
                }),
            ),
            VaultItem::new(
                ItemType::SecureNote,
                "Server SSH Key",
                ItemData::SecureNote(SecureNoteData {
                    content: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOrvpass... (Production Node)"
                        .to_string(),
                }),
            ),
            VaultItem::new(
                ItemType::CreditCard,
                "Corporate Card",
                ItemData::CreditCard(CreditCardData {
                    cardholder_name: "Dev Team".to_string(),
                    card_number: "4242424242424242".to_string(),
                    expiration_month: "12".to_string(),
                    expiration_year: "2028".to_string(),
                    cvv: "123".to_string(),
                }),
            ),
        ];
    }

    let password = get_master_password();
    if let Ok(raw) = fs::read(&path) {
        if raw.len() > 28 {
            let mut salt = [0u8; 16];
            salt.copy_from_slice(&raw[..16]);
            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&raw[16..28]);
            let ciphertext = &raw[28..];

            if let Ok(key) = derive_master_key(password.as_bytes(), &salt) {
                let enc_data = orvpass_core::crypto::EncryptedData {
                    nonce,
                    ciphertext: ciphertext.to_vec(),
                };
                if let Ok(decrypted) = decrypt(&key, &enc_data) {
                    if let Ok(payload) = serde_json::from_slice::<EncryptedVaultPayload>(&decrypted)
                    {
                        return payload.items;
                    }
                }
            }
        }
    }
    Vec::new()
}

pub fn save_items(items: &[VaultItem]) -> anyhow::Result<()> {
    let path = vault_path();
    let password = get_master_password();
    let salt = generate_salt();
    let key = derive_master_key(password.as_bytes(), &salt)?;

    let payload = EncryptedVaultPayload {
        version: 1,
        salt,
        items: items.to_vec(),
    };

    let serialized = serde_json::to_vec(&payload)?;
    let enc_data = encrypt(&key, &serialized)?;

    let mut final_buf = Vec::with_capacity(16 + 12 + enc_data.ciphertext.len());
    final_buf.extend_from_slice(&salt);
    final_buf.extend_from_slice(&enc_data.nonce);
    final_buf.extend_from_slice(&enc_data.ciphertext);

    fs::write(&path, final_buf)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

pub fn shred_file(path: &std::path::Path) -> std::io::Result<()> {
    if path.exists() {
        std::fs::write(path, [0u8; 1024])?;
        std::fs::remove_file(path)?;
    }
    Ok(())
}
