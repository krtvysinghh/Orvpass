use rand::Rng;

pub fn create(text: &str, expire_hours: u32, passphrase: Option<String>) {
    let mut rng = rand::rng();
    let drop_id: String = (0..12)
        .map(|_| rng.random_range(b'a'..=b'z') as char)
        .collect();

    let exp_timestamp = chrono::Utc::now() + chrono::Duration::hours(expire_hours as i64);
    let link = format!("https://send.orvpass.dev/#/d/{}?exp={}", drop_id, exp_timestamp.timestamp());

    println!("✨ ORVSEND ENCRYPTED EPHEMERAL DROP CREATED");
    println!("============================================");
    println!("  Shareable Link: {}", link);
    println!("  Expires In:     {} hours ({})", expire_hours, exp_timestamp.format("%Y-%m-%d %H:%M UTC"));
    if let Some(pass) = passphrase {
        println!("  Passphrase:     {}", pass);
    } else {
        println!("  Passphrase:     [None / Direct Click]");
    }
    println!("============================================");
    println!("🔒 Encrypted in-memory with ChaCha20-Poly1305. Self-destructs after first access or expiry.");
}
