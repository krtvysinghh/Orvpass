pub mod exporters;
pub mod formats;
pub mod parsers;

pub enum ImportFormat {
    Bitwarden,
    OnePassword,
    KeePass,
    LastPass,
    Dashlane,
    ProtonPass,
    RoboForm,
    Csv,
    Json,
}

pub enum ExportFormat {
    Json,
    Csv,
    Bitwarden,
    KeePass,
}

pub fn detect_vault_format(content: &str) -> &'static str {
    if content.contains("encryptedFor") || content.contains("ciphers") {
        "Bitwarden"
    } else if content.contains("KeePassFile") || content.contains("Group") {
        "KeePass"
    } else if content.contains("title,username,password") {
        "CSV"
    } else {
        "Orvpass JSON"
    }
}
