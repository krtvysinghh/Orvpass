use std::fs;
use std::path::Path;

pub fn backup(source: &Path, target: &Path) -> std::io::Result<()> {
    fs::copy(source, target)?;

    Ok(())
}

pub mod safety;
