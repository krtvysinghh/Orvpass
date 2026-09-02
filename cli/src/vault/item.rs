use orvpass_core::models::VaultItem;

pub fn merge_items(existing: &mut Vec<VaultItem>, new_items: Vec<VaultItem>, overwrite: bool) -> usize {
    let mut added = 0;
    for item in new_items {
        if let Some(pos) = existing.iter().position(|i| i.title.eq_ignore_ascii_case(&item.title)) {
            if overwrite {
                existing[pos] = item;
                added += 1;
            }
        } else {
            existing.push(item);
            added += 1;
        }
    }
    added
}
