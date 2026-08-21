use std::fs;
use std::path::Path;

pub fn export_file(source: &Path, target: &Path) -> std::io::Result<()> {
    fs::copy(source, target)?;

    Ok(())
}
