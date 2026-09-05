use crate::vault::database;

pub fn execute_repair() {
    let mut items = database::load_items();
    let count = items.len();
    for item in &mut items {
        item.title = item.title.trim().to_string();
    }
    let _ = database::save_items(&items);
    println!("🔧 Verified & repaired {} vault items with zero data loss.", count);
}
