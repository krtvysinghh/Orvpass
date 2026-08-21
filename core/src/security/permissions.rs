use std::fs;
use std::path::Path;

pub fn readable(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}
