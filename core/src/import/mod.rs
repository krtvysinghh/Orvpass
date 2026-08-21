use std::fs;
use std::path::Path;

pub fn import_file(source: &Path, target: &Path) -> std::io::Result<()> {
    fs::copy(source, target)?;

    Ok(())
}
