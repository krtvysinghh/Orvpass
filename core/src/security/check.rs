pub fn password_policy(password: &str) -> bool {
    validate_master(password)
}

pub fn validate_master(password: &str) -> bool {
    if password.len() < 12 {
        return false;
    }

    let lower = password.chars().any(|c| c.is_lowercase());
    let upper = password.chars().any(|c| c.is_uppercase());
    let digit = password.chars().any(|c| c.is_numeric());
    let symbol = password.chars().any(|c| !c.is_alphanumeric());

    lower && upper && digit && symbol
}

pub fn entropy_score(password: &str) -> usize {
    let mut score = 0;

    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    }

    if password.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    }

    score
}
