#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TotpConfig {
    pub digits: u8,
    pub period_seconds: u32,
    pub algorithm: TotpAlgorithm,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            digits: 6,
            period_seconds: 30,
            algorithm: TotpAlgorithm::Sha1,
        }
    }
}
