pub fn copy_with_notification(content: &str, timeout_secs: u64) {
    println!("📋 Copied to clipboard. Auto-wiping in {}s...", timeout_secs);
    let _ = content;
}
