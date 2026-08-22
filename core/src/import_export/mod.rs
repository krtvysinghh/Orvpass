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
