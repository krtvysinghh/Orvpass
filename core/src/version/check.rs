use super::constants::*;

pub fn check() -> bool {
    PRODUCT == "Orvpass" && !ENGINE_VERSION.is_empty()
}

pub fn is_release_ready() -> bool {
    CHANNEL == "stable" && check()
}
