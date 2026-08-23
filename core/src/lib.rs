pub mod build;
pub mod crypto;
pub mod models;
pub mod runtime;
pub mod security;
pub mod totp;
pub mod vault;
pub mod version;

pub use crypto::*;
pub use models::*;
pub use vault::*;

pub fn info() -> (&'static str, &'static str) {
    ("Orvpass", env!("CARGO_PKG_VERSION"))
}
