use crate::vault::database;
use orvpass_core::import_export::detect_vault_format;
use orvpass_core::import_export::parsers::{
    bitwarden::parse_bitwarden_json,
    chrome::parse_chrome_csv,
    apple::parse_apple_csv,
    lastpass::parse_lastpass_csv,
    onepassword::parse_1password_csv,
    keepass::parse_keepass_csv,
};
use std::fs;

pub fn execute(file: &str) {
    if let Ok(content) = fs::read_to_string(file) {
        let fmt = detect_vault_format(&content);
        println!("📥 Auto-detected format: {}", fmt);
        
        let mut new_items = match fmt {
            "Bitwarden" => parse_bitwarden_json(&content).unwrap_or_default(),
            "Chrome" => parse_chrome_csv(&content),
            "Apple" => parse_apple_csv(&content),
            "LastPass" => parse_lastpass_csv(&content),
            "1Password" => parse_1password_csv(&content),
            "KeePass" => parse_keepass_csv(&content),
            _ => vec![],
        };

        let count = new_items.len();
        if count > 0 {
            let mut existing = database::load_items();
            existing.append(&mut new_items);
            let _ = database::save_items(&existing);
            println!("✨ Successfully imported {} items from '{}' into encrypted vault.", count, file);
        } else {
            println!("ℹ️  File '{}' read successfully (0 valid credentials found).", file);
        }
    } else {
        println!("❌ Error: Unable to read file '{}'.", file);
    }
}
