pub fn score(password: &str) -> u8 {
    let mut s = 0;

    if password.len() >= 12 {
        s += 2;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        s += 1;
    }
    if password.chars().any(|c| c.is_lowercase()) {
        s += 1;
    }
    if password.chars().any(|c| c.is_numeric()) {
        s += 1;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        s += 1;
    }

    s
}
