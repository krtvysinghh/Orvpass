pub struct ImportMetadata {
    pub name: &'static str,
    pub version: &'static str,
}

pub fn supported_formats() -> Vec<&'static str> {
    vec![
        "Bitwarden",
        "1Password",
        "KeePass",
        "LastPass",
        "Dashlane",
        "Proton Pass",
        "RoboForm",
        "CSV",
        "JSON",
    ]
}
