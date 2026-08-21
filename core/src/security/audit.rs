use std::time::{SystemTime, UNIX_EPOCH};

pub fn log(event: &str) {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("[AUDIT {}] {}", ts, event);
}
