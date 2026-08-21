use orvpass_core::totp::{generate, TotpAlgorithm, TotpConfig, TotpError, TotpSecret};

#[test]
fn sha256_generation_is_stable() {
    let secret = TotpSecret::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ").unwrap();

    let config = TotpConfig {
        digits: 8,
        period_seconds: 30,
        algorithm: TotpAlgorithm::Sha256,
        skew_steps: 0,
    };

    let first = generate(&secret, 59, config).unwrap();
    let second = generate(&secret, 59, config).unwrap();

    assert_eq!(first.value(), second.value());
}

#[test]
fn sha512_generation_is_stable() {
    let secret = TotpSecret::new("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ").unwrap();

    let config = TotpConfig {
        digits: 8,
        period_seconds: 30,
        algorithm: TotpAlgorithm::Sha512,
        skew_steps: 0,
    };

    let first = generate(&secret, 59, config).unwrap();
    let second = generate(&secret, 59, config).unwrap();

    assert_eq!(first.value(), second.value());
}

#[test]
fn zero_timestamp_is_rejected() {
    let secret = TotpSecret::new("JBSWY3DPEHPK3PXP").unwrap();

    assert!(matches!(
        generate(&secret, 0, TotpConfig::default()),
        Err(TotpError::InvalidTimestamp)
    ));
}

#[test]
fn secret_does_not_appear_in_debug_output() {
    let secret = TotpSecret::new("JBSWY3DPEHPK3PXP").unwrap();
    let debug = format!("{secret:?}");

    assert!(!debug.contains("JBSWY3DPEHPK3PXP"));
}
