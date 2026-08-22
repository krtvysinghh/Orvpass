use std::sync::Mutex;

static SESSION: Mutex<Option<[u8; 32]>> = Mutex::new(None);

pub fn unlock(key: [u8; 32]) {
    let mut session = SESSION.lock().unwrap();

    *session = Some(key);
}

pub fn lock() {
    let mut session = SESSION.lock().unwrap();

    *session = None;
}

pub fn is_unlocked() -> bool {
    SESSION.lock().unwrap().is_some()
}

pub fn key() -> Option<[u8; 32]> {
    SESSION.lock().unwrap().clone()
}
