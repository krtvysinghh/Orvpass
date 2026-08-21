pub mod crypto;
pub mod models;
pub mod vault;

pub use crypto::*;
pub use models::*;
pub use vault::*;

pub fn info() -> (&'static str, &'static str) {
    ("Orvpass", env!("CARGO_PKG_VERSION"))
}
pub mod totp;

pub mod kdf;

pub mod security;

pub mod config;

pub mod backup;

pub mod clipboard;

pub mod export;

pub mod import;

pub mod errors;

pub mod runtime;

pub mod version;

pub mod logging;

pub mod memory;

pub mod result;
