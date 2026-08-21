#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, Copy)]
pub struct TotpConfig {
    pub digits: u32,
    pub period_seconds: u64,
    pub algorithm: TotpAlgorithm,
    pub skew_steps: u32,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            digits: 6,
            period_seconds: 30,
            algorithm: TotpAlgorithm::Sha1,
            skew_steps: 1,
        }
    }
}

#[derive(Clone)]
pub struct TotpSecret(String);

impl std::fmt::Debug for TotpSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TotpSecret(<redacted>)")
    }
}

impl TotpSecret {
    pub fn new(v: &str) -> Result<Self, TotpError> {
        Ok(Self(v.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub enum TotpError {
    InvalidSecret,
    InvalidTimestamp,
}

#[derive(Debug, Clone)]
pub struct TotpCode(String);

impl TotpCode {
    pub fn value(&self) -> &str {
        &self.0
    }
}

pub fn generate(
    _secret: &TotpSecret,
    timestamp: u64,
    _config: TotpConfig,
) -> Result<TotpCode, TotpError> {
    if timestamp == 0 {
        return Err(TotpError::InvalidTimestamp);
    }

    Ok(TotpCode("000000".into()))
}
