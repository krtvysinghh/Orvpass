pub mod exporters;
pub mod formats;
pub mod parsers;

pub fn detect_vault_format(content: &str) -> &'static str {
    if content.contains("encryptedFor") || content.contains("ciphers") || content.contains("\"items\":") {
        "Bitwarden"
    } else if content.contains("KeePassFile") || content.contains("Group") {
        "KeePass"
    } else if content.contains("otpauth") || content.contains("Proton") {
        "ProtonPass"
    } else if content.contains("Title,URL,Username,Password") {
        "Apple"
    } else if content.contains("name,url,username,password") {
        "Chrome"
    } else if content.contains("url,username,password,totp") {
        "LastPass"
    } else if content.contains("title,username,password") {
        "CSV"
    } else {
        "Orvpass JSON"
    }
}
