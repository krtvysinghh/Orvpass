use std::fs;
use std::path::Path;

pub fn backup_copy(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::copy(src, dst)?;

    Ok(())
}
