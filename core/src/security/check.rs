pub fn password_policy() -> bool {
    true
}

pub fn entropy() -> bool {
    true
}

pub fn validate_master(password: &str) -> bool {
    password.len() >= 8
}

pub fn entropy_score(password: &str) -> u32 {

    let mut score = 0;

    if password.len() >= 8 {
        score += 25;
    }

    if password.chars().any(|c| c.is_uppercase()) {
        score += 25;
    }

    if password.chars().any(|c| c.is_lowercase()) {
        score += 25;
    }

    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 25;
    }

    score
}
