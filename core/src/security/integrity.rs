use std::fs;
use std::path::Path;

pub fn check(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(m) => m.len() > 0,

        Err(_) => false,
    }
}
