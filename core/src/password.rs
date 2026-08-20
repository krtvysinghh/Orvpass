use rand::RngCore;
use zeroize::Zeroizing;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordError {
    InvalidLength,
    EmptyCharacterSet,
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

    pub fn entropy_bits(&self, config: PasswordConfig) -> f64 {
        let charset_size = charset(config).len() as f64;
        self.value.len() as f64 * charset_size.log2()
    }
}

pub fn generate(config: PasswordConfig) -> Result<GeneratedPassword, PasswordError> {
    if config.length == 0 {
        return Err(PasswordError::InvalidLength);
    }

    let chars = charset(config);

    if chars.is_empty() {
        return Err(PasswordError::EmptyCharacterSet);
    }

    let mut output = String::with_capacity(config.length);
    let mut buffer = [0u8; 8];
    let mut rng = rand::rng();

    while output.len() < config.length {
        rng.fill_bytes(&mut buffer);

        for byte in buffer {
            let index = byte as usize;
            let limit = 256 - (256 % chars.len());

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
        chars.extend(b"!@#$%^&*()-_=+[]{}:,.?");
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
}
