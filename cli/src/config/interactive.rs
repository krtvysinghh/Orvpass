use std::io::{self, Write};

pub fn prompt(msg: &str) -> String {
    print!("{}", msg);

    io::stdout().flush().ok();

    let mut input = String::new();

    io::stdin().read_line(&mut input).ok();

    input.trim().to_string()
}
