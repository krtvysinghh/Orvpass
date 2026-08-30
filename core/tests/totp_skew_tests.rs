use orvpass_core::totp::generate_totp;

#[test]
fn test_totp_step_boundaries() {
    let secret = b"TEST_TOTP_SECRET_2026";
    let code1 = generate_totp(secret, 30).unwrap();
    assert!(code1 >= 100_000 && code1 <= 999_999);
}
