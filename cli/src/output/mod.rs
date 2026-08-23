pub fn header(msg: &str) {
    println!("\n🔐 {}\n{}", msg, "─".repeat(msg.len() + 4));
}

pub fn success(msg: &str) {
    println!("✓ {}", msg);
}

pub fn info(msg: &str) {
    println!("• {}", msg);
}

pub fn warning(msg: &str) {
    println!("⚠ {}", msg);
}

pub fn error(msg: &str) {
    eprintln!("✗ {}", msg);
}

pub fn table(headers: &[&str], rows: &[Vec<String>]) {
    println!("{}", headers.join(" | "));
    println!("{}", "-".repeat(40));
    for row in rows {
        println!("{}", row.join(" | "));
    }
}
