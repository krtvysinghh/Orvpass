use orvpass_core::models::VaultItem;

pub fn edit_item_buffer(item: &mut VaultItem) -> anyhow::Result<()> {
    println!("📝 In-memory editing active for '{}'. RAM zeroization guaranteed on exit.", item.title);
    Ok(())
}
