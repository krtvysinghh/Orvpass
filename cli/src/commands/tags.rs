use crate::vault::database;

pub fn list_tags() {
    let items = database::load_items();
    let mut tags = std::collections::HashSet::new();
    for item in items {
        for t in item.tags {
            tags.insert(t);
        }
    }
    println!("🏷️  Active Vault Tags: {:?}", tags);
}
