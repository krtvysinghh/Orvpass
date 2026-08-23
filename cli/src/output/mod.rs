use std::io::{self, Write};

pub fn success(msg: &str) {
    println!("✓ {}", msg);
}

pub fn info(msg: &str) {
    println!("• {}", msg);
}

pub fn warning(msg: &str) {
    eprintln!("⚠ {}", msg);
}

pub fn error(msg: &str) {
    eprintln!("✗ {}", msg);
}

pub fn header(title: &str) {
    println!();
    println!("━━ {} ━━", title);
}

pub fn table(headers: &[&str], rows: &[Vec<String>]) {
    let widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            rows.iter()
                .map(|r| r.get(i).map(|x| x.len()).unwrap_or(0))
                .max()
                .unwrap_or(0)
                .max(h.len())
        })
        .collect();

    for (i, h) in headers.iter().enumerate() {
        print!("{:<width$} ", h, width = widths[i]);
    }
    println!();

    for w in &widths {
        print!("{:-<width$} ", "", width = *w);
    }
    println!();

    for row in rows {
        for (i, col) in row.iter().enumerate() {
            print!("{:<width$} ", col, width = widths[i]);
        }
        println!();
    }

    io::stdout().flush().ok();
}

pub struct Spinner {
    message: String,
}

impl Spinner {
    pub fn start(message: &str) -> Self {
        println!("⠋ {}", message);
        Self {
            message: message.to_string(),
        }
    }

    pub fn finish(self, message: &str) {
        println!("✓ {}", message);
        let _ = self.message;
    }
}
