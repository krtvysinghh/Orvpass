use rand::RngCore;
use zeroize::Zeroizing;

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 256;
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}:,.?";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PasswordConfig {
    pub length: usize,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 20,
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PasswordPolicy {
    pub minimum_entropy_bits: f64,
    pub minimum_length: usize,
    pub maximum_length: usize,
    pub require_lowercase: bool,
    pub require_uppercase: bool,
    pub require_digits: bool,
    pub require_symbols: bool,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            minimum_entropy_bits: 100.0,
            minimum_length: 12,
            maximum_length: 256,
            require_lowercase: true,
            require_uppercase: true,
            require_digits: true,
            require_symbols: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordError {
    InvalidLength,
    EmptyCharacterSet,
    EntropyTooLow,
    PolicyViolation,
}

#[derive(Clone)]
pub struct GeneratedPassword {
    value: Zeroizing<String>,
}

impl std::fmt::Debug for GeneratedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeneratedPassword")
            .field("value", &"[REDACTED]")
            .finish()
    }
}

impl GeneratedPassword {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn validate(
        &self,
        config: PasswordConfig,
        policy: PasswordPolicy,
    ) -> Result<(), PasswordError> {
        let length = self.value.len();

        if length < policy.minimum_length || length > policy.maximum_length {
            return Err(PasswordError::PolicyViolation);
        }

        if self.entropy_bits(config) < policy.minimum_entropy_bits {
            return Err(PasswordError::EntropyTooLow);
        }

        if policy.require_lowercase && !self.value.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(PasswordError::PolicyViolation);
        }

        if policy.require_uppercase && !self.value.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(PasswordError::PolicyViolation);
        }

        if policy.require_digits && !self.value.chars().any(|c| c.is_ascii_digit()) {
            return Err(PasswordError::PolicyViolation);
        }

        if policy.require_symbols && !self.value.chars().any(|c| !c.is_ascii_alphanumeric()) {
            return Err(PasswordError::PolicyViolation);
        }

        Ok(())
    }

    pub fn entropy_bits(&self, config: PasswordConfig) -> f64 {
        let charset_size = charset(config).len();

        if charset_size == 0 {
            return 0.0;
        }

        self.value.len() as f64 * (charset_size as f64).log2()
    }
}

pub fn generate(config: PasswordConfig) -> Result<GeneratedPassword, PasswordError> {
    validate_config(config)?;

    let chars = charset(config);

    let mut output = String::with_capacity(config.length);
    let mut buffer = [0u8; 8];
    let mut rng = rand::rng();

    let limit = 256 - (256 % chars.len());

    while output.len() < config.length {
        rng.fill_bytes(&mut buffer);

        for byte in buffer {
            let index = byte as usize;

            if index >= limit {
                continue;
            }

            output.push(chars[index % chars.len()] as char);

            if output.len() == config.length {
                break;
            }
        }
    }

    Ok(GeneratedPassword {
        value: Zeroizing::new(output),
    })
}

fn validate_config(config: PasswordConfig) -> Result<(), PasswordError> {
    if !(MIN_PASSWORD_LENGTH..=MAX_PASSWORD_LENGTH).contains(&config.length) {
        return Err(PasswordError::InvalidLength);
    }

    if charset(config).is_empty() {
        return Err(PasswordError::EmptyCharacterSet);
    }

    Ok(())
}

fn charset(config: PasswordConfig) -> Vec<u8> {
    let mut chars = Vec::with_capacity(94);

    if config.lowercase {
        chars.extend(b'a'..=b'z');
    }

    if config.uppercase {
        chars.extend(b'A'..=b'Z');
    }

    if config.digits {
        chars.extend(b'0'..=b'9');
    }

    if config.symbols {
        chars.extend(SYMBOLS);
    }

    chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_password_has_expected_length() {
        let password = generate(PasswordConfig::default()).unwrap();

        assert_eq!(password.value().len(), 20);
    }

    #[test]
    fn generated_passwords_are_different() {
        let config = PasswordConfig::default();

        let first = generate(config).unwrap();
        let second = generate(config).unwrap();

        assert_ne!(first.value(), second.value());
    }

    #[test]
    fn zero_length_is_rejected() {
        let config = PasswordConfig {
            length: 0,
            ..PasswordConfig::default()
        };

        assert!(matches!(
            generate(config),
            Err(PasswordError::InvalidLength)
        ));
    }

    #[test]
    fn length_below_policy_is_rejected() {
        let config = PasswordConfig {
            length: MIN_PASSWORD_LENGTH - 1,
            ..PasswordConfig::default()
        };

        assert!(matches!(
            generate(config),
            Err(PasswordError::InvalidLength)
        ));
    }

    #[test]
    fn length_above_policy_is_rejected() {
        let config = PasswordConfig {
            length: MAX_PASSWORD_LENGTH + 1,
            ..PasswordConfig::default()
        };

        assert!(matches!(
            generate(config),
            Err(PasswordError::InvalidLength)
        ));
    }

    #[test]
    fn policy_boundaries_are_accepted() {
        for length in [MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH] {
            let config = PasswordConfig {
                length,
                ..PasswordConfig::default()
            };

            let password = generate(config).unwrap();

            assert_eq!(password.value().len(), length);
        }
    }

    #[test]
    fn empty_charset_is_rejected() {
        let config = PasswordConfig {
            length: 20,
            lowercase: false,
            uppercase: false,
            digits: false,
            symbols: false,
        };

        assert!(matches!(
            generate(config),
            Err(PasswordError::EmptyCharacterSet)
        ));
    }

    #[test]
    fn lowercase_only_contains_lowercase() {
        let config = PasswordConfig {
            length: 32,
            lowercase: true,
            uppercase: false,
            digits: false,
            symbols: false,
        };

        let password = generate(config).unwrap();

        assert!(password.value().bytes().all(|b| b.is_ascii_lowercase()));
    }

    #[test]
    fn digits_only_contains_digits() {
        let config = PasswordConfig {
            length: 32,
            lowercase: false,
            uppercase: false,
            digits: true,
            symbols: false,
        };

        let password = generate(config).unwrap();

        assert!(password.value().bytes().all(|b| b.is_ascii_digit()));
    }

    #[test]
    fn debug_output_redacts_password() {
        let password = generate(PasswordConfig::default()).unwrap();
        let debug = format!("{password:?}");

        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(password.value()));
    }

    #[test]
    fn entropy_is_positive() {
        let config = PasswordConfig::default();
        let password = generate(config).unwrap();

        assert!(password.entropy_bits(config) > 100.0);
    }

    #[test]
    fn entropy_increases_with_length() {
        let short = PasswordConfig {
            length: 12,
            ..PasswordConfig::default()
        };

        let long = PasswordConfig {
            length: 24,
            ..PasswordConfig::default()
        };

        let short_password = generate(short).unwrap();
        let long_password = generate(long).unwrap();

        assert!(long_password.entropy_bits(long) > short_password.entropy_bits(short));
    }

    #[test]
    fn strong_password_passes_policy() {
        let config = PasswordConfig {
            length: 24,
            ..Default::default()
        };

        let password = generate(config).unwrap();

        assert!(password.validate(config, PasswordPolicy::default()).is_ok());
    }

    #[test]
    fn weak_password_fails_entropy_policy() {
        let config = PasswordConfig {
            length: 8,
            lowercase: true,
            uppercase: false,
            digits: false,
            symbols: false,
        };

        let password = GeneratedPassword {
            value: Zeroizing::new("abcdefgh".to_string()),
        };

        let policy = PasswordPolicy {
            minimum_entropy_bits: 200.0,
            minimum_length: 8,
            maximum_length: MAX_PASSWORD_LENGTH,
            require_lowercase: false,
            require_uppercase: false,
            require_digits: false,
            require_symbols: false,
        };

        assert!(password.entropy_bits(config) < 200.0);

        assert!(matches!(
            password.validate(config, policy),
            Err(PasswordError::EntropyTooLow)
        ));
    }
}
