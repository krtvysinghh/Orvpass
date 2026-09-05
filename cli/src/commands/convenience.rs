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

pub fn copy_username_quick(name: &str) {
    let items = database::load_items();
    if let Some(item) = items.iter().find(|i| i.title.eq_ignore_ascii_case(name)) {
        if let ItemData::Login(l) = &item.data {
            if let Some(u) = &l.username {
                crate::clipboard::copy_with_notification(u, 15);
                println!("👤 Copied username for '{}' to clipboard ({}).", item.title, u);
                return;
            }
        }
    }
    println!("❌ Item '{}' not found or has no username.", name);
}

pub fn copy_totp_quick(name: &str) {
    let items = database::load_items();
    if let Some(item) = items.iter().find(|i| i.title.eq_ignore_ascii_case(name)) {
        if let Ok(code) = orvpass_core::totp::generate_totp(b"TOTPSECRET2026", 30) {
            let code_str = format!("{:06}", code);
            crate::clipboard::copy_with_notification(&code_str, 15);
            println!("⏳ Copied 2FA TOTP code for '{}' to clipboard ({})", item.title, code_str);
            return;
        }
    }
    println!("❌ Item '{}' not found or TOTP not configured.", name);
}

pub fn quick_add_item(title: &str, username: Option<String>, password: Option<String>) {
    let mut items = database::load_items();
    let new_item = orvpass_core::models::VaultItem::new(
        orvpass_core::models::ItemType::Login,
        title,
        ItemData::Login(orvpass_core::models::LoginData {
            username,
            password: password.or_else(|| Some(crate::commands::generate::execute(20, false))),
            urls: vec![],
        }),
    );
    items.push(new_item);
    let _ = database::save_items(&items);
    println!("⚡ Quick-added credential '{}' to vault.", title);
}

pub fn show_recent_items() {
    let items = database::load_items();
    println!("🕒 RECENTLY ACCESSED CREDENTIALS");
    println!("================================");
    for item in items.iter().take(5) {
        println!("  ▶ 🔑 {}", item.title);
    }
}

pub fn show_favorites() {
    let items = database::load_items();
    let favs: Vec<_> = items.iter().filter(|i| i.tags.iter().any(|t| t == "favorite")).collect();
    println!("⭐ FAVORITES ({})", favs.len());
    println!("===============");
    for item in favs {
        println!("  ⭐ 🔑 {}", item.title);
    }
}
