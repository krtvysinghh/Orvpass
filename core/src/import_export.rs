#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    Bitwarden,
    ProtonPass,
    OnePassword,
    KeePass,
    LastPass,
    Dashlane,
    Chrome,
    Safari,
    Firefox,
    Csv,
    Json,
}
