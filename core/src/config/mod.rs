use std::path::PathBuf;

pub fn config_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap())
        .join(".orvpass")
        .join("config.toml")
}
