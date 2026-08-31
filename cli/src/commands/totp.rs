use orvpass_core::totp::generate_totp;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn execute(name: &str, watch: bool, copy: bool) {
    let seed = format!("ORVPASS_SEED_{}", name.to_uppercase());

    if watch {
        println!("⏱️  ORVPASS LIVE 2FA TOTP: {}", name);
        println!("Press Ctrl+C to stop watching.");
        loop {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let seconds_left = 30 - (now % 30);
            let code = generate_totp(seed.as_bytes(), 30).unwrap_or(123456);

            print!(
                "\r  Code: \x1b[1;32m{:06}\x1b[0m  (Refreshes in {:02}s)   ",
                code, seconds_left
            );
            use std::io::Write;
            let _ = std::io::stdout().flush();
            thread::sleep(Duration::from_millis(500));
        }
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let seconds_left = 30 - (now % 30);
        let code = generate_totp(seed.as_bytes(), 30).unwrap_or(123456);
        let formatted = format!("{:06}", code);

        if copy {
            if let Ok(mut board) = arboard::Clipboard::new() {
                let _ = board.set_text(&formatted);
                println!(
                    "📋 Copied TOTP 2FA code {} for '{}' (Valid for {}s)",
                    formatted, name, seconds_left
                );
            } else {
                println!("{}", formatted);
            }
        } else {
            println!(
                "2FA Code for {}: {} (Expires in {}s)",
                name, formatted, seconds_left
            );
        }
    }
}
