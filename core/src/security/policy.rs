pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_numbers: bool,
    pub require_symbols: bool,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 16,
            require_numbers: true,
            require_symbols: true,
        }
    }
}

impl PasswordPolicy {
    pub fn validate(&self, password: &str) -> bool {
        if password.len() < self.min_length {
            return false;
        }

        if self.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return false;
        }

        if self.require_symbols && password.chars().all(|c| c.is_alphanumeric()) {
            return false;
        }

        true
    }
}
