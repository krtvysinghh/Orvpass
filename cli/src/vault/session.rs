use orvpass_core::crypto::SecretKey;
use std::sync::Mutex;

static SESSION: Mutex<Option<SecretKey>> = Mutex::new(None);

pub fn set(key: SecretKey) {
    *SESSION.lock().unwrap() = Some(key);
}

pub fn unlock(key: SecretKey) {
    set(key);
}

pub fn lock() {
    *SESSION.lock().unwrap() = None;
}

pub fn is_unlocked() -> bool {
    SESSION.lock().unwrap().is_some()
}

pub fn unlocked() -> bool {
    is_unlocked()
}

pub fn key() -> Option<SecretKey> {
    SESSION.lock().unwrap().clone()
}
