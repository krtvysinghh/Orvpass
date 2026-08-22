use super::constants::*;

pub fn check() -> bool {
    PRODUCT == "Orvpass" && ENGINE_VERSION.starts_with("2.")
}

pub fn is_release_ready() -> bool {
    CHANNEL == "stable" && check()
}
