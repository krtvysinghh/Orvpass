use arboard::Clipboard;

pub fn run(value: String) {
    let mut clipboard = Clipboard::new().expect("clipboard unavailable");

    clipboard.set_text(value).expect("copy failed");

    println!("✓ Copied");
}
