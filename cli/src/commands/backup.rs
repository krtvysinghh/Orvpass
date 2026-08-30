use std::fs;
use crate::vault::database;

pub fn create_snapshot() -> anyhow::Result<()> {
    let src = database::vault_path();
    if src.exists() {
        let ts = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let dest = database::vault_dir().join(format!("vault_backup_{}.enc", ts));
        fs::copy(&src, &dest)?;
        println!("💾 Created encrypted snapshot: {}", dest.display());
    }
    Ok(())
}
