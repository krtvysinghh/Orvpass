use crate::vault::database;
use crate::clipboard;
use orvpass_core::models::ItemData;

pub fn copy_password_quick(name: &str) {
    let items = database::load_items();
    if let Some(item) = items.iter().find(|i| i.title.eq_ignore_ascii_case(name)) {
        if let ItemData::Login(l) = &item.data {
            if let Some(p) = &l.password {
                clipboard::copy_with_notification(p, 15);
                println!("🔑 Copied password for '{}' to clipboard (auto-wipes in 15s).", item.title);
                return;
            }
        }
    }
    println!("❌ Item '{}' not found or has no password.", name);
}
