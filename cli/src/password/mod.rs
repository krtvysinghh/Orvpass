use rand::Rng;

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}";

pub fn generate(length: usize, numbers: bool, symbols: bool) -> String {
    let mut charset = format!("{}{}", LOWER, UPPER);

    if numbers {
        charset.push_str(NUMBERS);
    }

    if symbols {
        charset.push_str(SYMBOLS);
    }

    let chars: Vec<char> = charset.chars().collect();

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

pub fn entropy(length: usize, charset: usize) -> f64 {
    (length as f64) * (charset as f64).log2()
}
