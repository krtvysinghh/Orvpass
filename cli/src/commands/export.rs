use crate::vault::database;
use orvpass_core::import_export::exporters::html::export_standalone_html;
use std::fs;

pub fn execute(file: &str) {
    let items = database::load_items();
    if file.ends_with(".html") {
        let html = export_standalone_html(&items);
        let _ = fs::write(file, html);
        println!("🌐 Exported standalone offline HTML vault to '{}' ({} items)", file, items.len());
    } else if file.ends_with(".json") {
        let json = serde_json::to_string_pretty(&items).unwrap_or_default();
        let _ = fs::write(file, json);
        println!("📄 Exported JSON vault to '{}' ({} items)", file, items.len());
    } else {
        let json = serde_json::to_string_pretty(&items).unwrap_or_default();
        let _ = fs::write(file, json);
        println!("💾 Exported vault to '{}' ({} items)", file, items.len());
    }
}
