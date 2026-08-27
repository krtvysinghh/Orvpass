use crate::vault::database;

pub fn execute(name: String) -> anyhow::Result<()> {
    let mut items = database::load_items();
    let initial_len = items.len();
    items.retain(|i| !i.title.eq_ignore_ascii_case(&name) && !i.name.eq_ignore_ascii_case(&name));

    if items.len() < initial_len {
        database::save_items(&items)?;
        println!("🗑️  Deleted '{}' from vault.", name);
    } else {
        println!("❌ Item '{}' not found in vault.", name);
    }
    Ok(())
}
