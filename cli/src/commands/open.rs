use crate::vault::database;
use orvpass_core::models::ItemData;

pub fn open_browser(name: &str) {
    let items = database::load_items();
    if let Some(item) = items.iter().find(|i| i.title.eq_ignore_ascii_case(name)) {
        if let ItemData::Login(l) = &item.data {
            if let Some(url) = l.urls.first() {
                println!("🌐 Opening URL '{}' in default browser...", url);
                #[cfg(target_os = "macos")]
                let _ = std::process::Command::new("open").arg(url).spawn();
                #[cfg(target_os = "linux")]
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
                return;
            }
        }
    }
    println!("❌ No URL found for item '{}'.", name);
}
