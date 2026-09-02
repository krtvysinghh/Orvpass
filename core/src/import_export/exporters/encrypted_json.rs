use crate::models::VaultItem;

pub fn export_encrypted_json_envelope(items: &[VaultItem], salt_hex: &str) -> String {
    let payload = serde_json::json!({
        "generator": "Orvpass Enterprise",
        "version": 1,
        "kdf": "Argon2id",
        "cipher": "ChaCha20-Poly1305",
        "salt": salt_hex,
        "items_count": items.len()
    });
    serde_json::to_string_pretty(&payload).unwrap_or_default()
}
