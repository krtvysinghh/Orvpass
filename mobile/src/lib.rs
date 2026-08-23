
use std::sync::Mutex;

static STATE: Mutex<bool> = Mutex::new(false);

#[no_mangle]
pub extern "C" fn orvpass_version() -> *const u8 {
    b"3.1.0\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn orvpass_unlock() -> bool {
    let mut state = STATE.lock().unwrap();
    *state = true;
    true
}

#[no_mangle]
pub extern "C" fn orvpass_lock() {
    let mut state = STATE.lock().unwrap();
    *state = false;
}
