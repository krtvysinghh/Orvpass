pub mod database;
pub mod encryption;
pub mod item;
pub mod session;
pub mod store;

use orvpass_core::vault::Vault;
use std::path::PathBuf;

pub fn path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap())
        .join(".orvpass")
        .join("vault.orv")
}

pub fn open() -> Vault {
    Vault::new(path())
}
