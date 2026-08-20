use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use zeroize::{Zeroize, Zeroizing};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TotpConfig {
    pub digits: u32,
    pub period_seconds: u64,
    pub algorithm: TotpAlgorithm,
    pub skew_steps: i64,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TotpError {
    EmptySecret,
    InvalidSecret,
    InvalidDigits,
    InvalidPeriod,
    InvalidTimestamp,
    InvalidCode,
}

impl std::fmt::Display for TotpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::EmptySecret => "TOTP secret is empty",
            Self::InvalidSecret => "TOTP secret is invalid",
            Self::InvalidDigits => "TOTP digits must be 6 or 8",
            Self::InvalidPeriod => "TOTP period must be greater than zero",
            Self::InvalidTimestamp => "TOTP timestamp is invalid",
            Self::InvalidCode => "TOTP code is invalid",
        })
    }
}

impl std::error::Error for TotpError {}

pub struct TotpSecret {
    encoded: Zeroizing<String>,
}

impl std::fmt::Debug for TotpSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TotpSecret")
            .field("encoded", &"[REDACTED]")
            .finish()
    }
}

impl Clone for TotpSecret {
    fn clone(&self) -> Self {
        Self {
            encoded: Zeroizing::new(self.encoded.to_string()),
        }
    }
}

impl TotpSecret {
    pub fn new(secret: impl Into<String>) -> Result<Self, TotpError> {
        let secret = secret.into().trim().replace(' ', "").to_ascii_uppercase();

        if secret.is_empty() {
            return Err(TotpError::EmptySecret);
        }

        let normalized = if secret.len() % 8 == 0 {
            secret
        } else {
            format!("{}{}", secret, "=".repeat((8 - secret.len() % 8) % 8))
        };

        base32::decode(base32::Alphabet::Rfc4648 { padding: true }, &normalized)
            .filter(|bytes| !bytes.is_empty())
            .ok_or(TotpError::InvalidSecret)?;

        Ok(Self {
            encoded: Zeroizing::new(normalized.trim_end_matches('=').to_owned()),
        })
    }

    pub fn encoded(&self) -> &str {
        &self.encoded
    }

    fn bytes(&self) -> Result<Zeroizing<Vec<u8>>, TotpError> {
        let padded = if self.encoded.len().is_multiple_of(8) {
            self.encoded.to_string()
        } else {
            format!(
                "{}{}",
                self.encoded.as_str(),
                "=".repeat((8 - self.encoded.len() % 8) % 8)
            )
        };

        base32::decode(base32::Alphabet::Rfc4648 { padding: true }, &padded)
            .filter(|bytes| !bytes.is_empty())
            .map(Zeroizing::new)
            .ok_or(TotpError::InvalidSecret)
    }
}

impl Drop for TotpSecret {
    fn drop(&mut self) {
        self.encoded.zeroize();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TotpCode {
    value: String,
    remaining_seconds: u64,
}

impl TotpCode {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn remaining_seconds(&self) -> u64 {
        self.remaining_seconds
    }
}

pub fn generate(
    secret: &TotpSecret,
    timestamp: u64,
    config: TotpConfig,
) -> Result<TotpCode, TotpError> {
    validate_config(config)?;

    if timestamp == 0 {
        return Err(TotpError::InvalidTimestamp);
    }

    let counter = timestamp / config.period_seconds;
    let code = hotp(secret, counter, config)?;

    Ok(TotpCode {
        value: code,
        remaining_seconds: config.period_seconds - (timestamp % config.period_seconds),
    })
}

pub fn verify(
    secret: &TotpSecret,
    code: &str,
    timestamp: u64,
    config: TotpConfig,
) -> Result<bool, TotpError> {
    validate_config(config)?;

    if timestamp == 0 {
        return Err(TotpError::InvalidTimestamp);
    }

    if code.len() != config.digits as usize || !code.bytes().all(|b| b.is_ascii_digit()) {
        return Err(TotpError::InvalidCode);
    }

    let counter = timestamp / config.period_seconds;

    for offset in -config.skew_steps..=config.skew_steps {
        let candidate = if offset.is_negative() {
            counter.saturating_sub(offset.unsigned_abs())
        } else {
            counter.saturating_add(offset as u64)
        };

        let expected = hotp(secret, candidate, config)?;

        if constant_time_equal(expected.as_bytes(), code.as_bytes()) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn validate_config(config: TotpConfig) -> Result<(), TotpError> {
    if config.digits != 6 && config.digits != 8 {
        return Err(TotpError::InvalidDigits);
    }

    if config.period_seconds == 0 {
        return Err(TotpError::InvalidPeriod);
    }

    Ok(())
}

fn hotp(secret: &TotpSecret, counter: u64, config: TotpConfig) -> Result<String, TotpError> {
    let key = secret.bytes()?;
    let counter_bytes = counter.to_be_bytes();

    let digest = match config.algorithm {
        TotpAlgorithm::Sha1 => {
            let mut mac =
                Hmac::<Sha1>::new_from_slice(&key).map_err(|_| TotpError::InvalidSecret)?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        TotpAlgorithm::Sha256 => {
            let mut mac =
                Hmac::<Sha256>::new_from_slice(&key).map_err(|_| TotpError::InvalidSecret)?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        TotpAlgorithm::Sha512 => {
            let mut mac =
                Hmac::<Sha512>::new_from_slice(&key).map_err(|_| TotpError::InvalidSecret)?;
            mac.update(&counter_bytes);
            mac.finalize().into_bytes().to_vec()
        }
    };

    let offset = (digest[digest.len() - 1] & 0x0f) as usize;

    let binary = ((digest[offset] as u32 & 0x7f) << 24)
        | ((digest[offset + 1] as u32) << 16)
        | ((digest[offset + 2] as u32) << 8)
        | digest[offset + 3] as u32;

    let modulus = 10u32.pow(config.digits);
    let value = binary % modulus;

    Ok(format!("{value:0width$}", width = config.digits as usize))
}

fn constant_time_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut difference = 0u8;

    for (left, right) in a.iter().zip(b.iter()) {
        difference |= left ^ right;
    }

    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_standard() {
        let config = TotpConfig::default();

        assert_eq!(config.digits, 6);
        assert_eq!(config.period_seconds, 30);
        assert_eq!(config.algorithm, TotpAlgorithm::Sha1);
        assert_eq!(config.skew_steps, 1);
    }

    #[test]
    fn secret_normalizes_spaces_and_case() {
        let secret = TotpSecret::new(" jbsw y3dp ehpk 3pxp ").unwrap();

        assert_eq!(secret.encoded(), "JBSWY3DPEHPK3PXP");
    }

    #[test]
    fn rfc6238_sha1_vector() {
        let secret = TotpSecret::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ").unwrap();

        let config = TotpConfig {
            digits: 8,
            period_seconds: 30,
            algorithm: TotpAlgorithm::Sha1,
            skew_steps: 0,
        };

        let code = generate(&secret, 59, config).unwrap();

        assert_eq!(code.value(), "94287082");
        assert_eq!(code.remaining_seconds(), 1);
    }

    #[test]
    fn verification_accepts_current_code() {
        let secret = TotpSecret::new("JBSWY3DPEHPK3PXP").unwrap();
        let config = TotpConfig::default();

        let generated = generate(&secret, 1_700_000_000, config).unwrap();

        assert!(verify(&secret, generated.value(), 1_700_000_000, config).unwrap());
    }

    #[test]
    fn verification_rejects_wrong_code() {
        let secret = TotpSecret::new("JBSWY3DPEHPK3PXP").unwrap();

        assert!(!verify(&secret, "000000", 1_700_000_000, TotpConfig::default()).unwrap());
    }

    #[test]
    fn verification_allows_one_step_clock_skew() {
        let secret = TotpSecret::new("JBSWY3DPEHPK3PXP").unwrap();
        let config = TotpConfig::default();

        let code = generate(&secret, 1_700_000_000, config).unwrap();

        assert!(verify(&secret, code.value(), 1_700_000_029, config).unwrap());
    }

    #[test]
    fn invalid_secret_is_rejected() {
        assert!(matches!(
            TotpSecret::new("NOT-A-VALID-BASE32-SECRET"),
            Err(TotpError::InvalidSecret)
        ));
    }

    #[test]
    fn invalid_configuration_is_rejected() {
        let secret = TotpSecret::new("JBSWY3DPEHPK3PXP").unwrap();

        let config = TotpConfig {
            digits: 7,
            ..TotpConfig::default()
        };

        assert!(matches!(
            generate(&secret, 1_700_000_000, config),
            Err(TotpError::InvalidDigits)
        ));
    }
}
