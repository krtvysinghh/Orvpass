pub fn info(msg: &str) {
    println!("ℹ️  {}", msg);
}

pub fn print_import_summary(total: usize, logins: usize, notes: usize) {
    println!(
        "📊 Import Summary: {} total ({} logins, {} secure notes)",
        total, logins, notes
    );
}

pub fn print_highlighted_json(json_str: &str) {
    println!("{}", json_str);
}
