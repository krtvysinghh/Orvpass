use crate::models::{ItemData, ItemType, LoginData, VaultItem};

pub fn parse_opera_csv(csv_str: &str) -> Vec<VaultItem> {
    crate::import_export::parsers::chrome::parse_chrome_csv(csv_str)
}
