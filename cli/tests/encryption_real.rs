#[path = "../src/vault/encryption.rs"]
mod encryption;

#[test]
fn aes_argon2_roundtrip() {
    let encrypted = encryption::encrypt("hello", "password");

    let decrypted = encryption::decrypt(&encrypted, "password").unwrap();

    assert_eq!(decrypted, "hello");
}

#[test]
fn wrong_password() {
    let encrypted = encryption::encrypt("secret", "right");

    assert!(encryption::decrypt(&encrypted, "wrong").is_none());
}
