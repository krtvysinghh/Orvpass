use std::fs;
use std::path::PathBuf;

fn path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".orvpass")
        .join("vault.enc")
}

pub fn save(data: &str) {
    let p = path();
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, data).unwrap();
}

pub fn load() -> String {
    fs::read_to_string(path()).unwrap_or_default()
}
