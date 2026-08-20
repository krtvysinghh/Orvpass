#![forbid(unsafe_code)]

pub const PRODUCT_NAME: &str = "Orvpass";
pub const PRODUCT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod crypto;
pub mod database;
pub mod generator;
pub mod import_export;
pub mod models;
pub mod passkeys;
pub mod search;
pub mod sync;
pub mod totp;
pub mod vault;

pub fn info() -> (&'static str, &'static str) {
    (PRODUCT_NAME, PRODUCT_VERSION)
}
