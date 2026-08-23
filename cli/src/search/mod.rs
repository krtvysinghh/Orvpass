use orvpass_core::models::VaultItem;

pub fn search(items: &[VaultItem], query: &str) -> Vec<VaultItem> {
    let q = query.to_lowercase();

    items
        .iter()
        .filter(|item| {
            item.name.to_lowercase().contains(&q)
                || item.title.to_lowercase().contains(&q)
                || item.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
        .cloned()
        .collect()
}
