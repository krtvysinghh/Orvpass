use std::thread;
use std::time::Duration;

pub fn clear_after(seconds: u64) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds));
    });
}
