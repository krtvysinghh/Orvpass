use std::fs;
use std::path::PathBuf;

use crate::vault::item::VaultItem;

fn path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".orvpass")
        .join("vault.json")
}

pub fn save(items: &[VaultItem]) {
    let p = path();
    fs::create_dir_all(p.parent().unwrap()).unwrap();

    let data = serde_json::to_string_pretty(items).unwrap();
    fs::write(p, data).unwrap();
}

pub fn load() -> Vec<VaultItem> {
    let data = fs::read_to_string(path()).unwrap_or_default();

    if data.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&data).unwrap_or_default()
    }
}
