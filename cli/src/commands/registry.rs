pub fn available() {
    let commands = [
        "init", "add", "list", "get", "edit", "delete", "search", "generate", "status", "doctor",
        "health", "info",
    ];

    println!("Commands:");

    for c in commands {
        println!("- {}", c);
    }
}
