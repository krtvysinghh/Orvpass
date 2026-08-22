use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct TotpSecret {
    value: String,
}

impl TotpSecret {
    pub fn new(value: &str) -> Result<Self, TotpError> {
        Ok(Self {
            value: value.to_string(),
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TotpConfig {
    pub digits: u32,
    pub period_seconds: u64,
    pub skew_steps: u32,
    pub algorithm: TotpAlgorithm,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            digits: 6,
            period_seconds: 30,
            skew_steps: 0,
            algorithm: TotpAlgorithm::Sha1,
        }
    }
}

#[derive(Debug)]
pub enum TotpError {
    InvalidTimestamp,
}

pub fn generate(
    secret: &TotpSecret,
    timestamp: u64,
    _config: TotpConfig,
) -> Result<TotpSecret, TotpError> {
    if timestamp == 0 {
        return Err(TotpError::InvalidTimestamp);
    }

    Ok(secret.clone())
}

impl std::fmt::Debug for TotpSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TotpSecret")
            .field("value", &"[REDACTED]")
            .finish()
    }
}
