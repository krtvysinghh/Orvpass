#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PasswordPolicy {
    pub length: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            length: 24,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: true,
        }
    }
}
