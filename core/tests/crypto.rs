use orvpass_core::crypto::{
    SecretKey, decrypt, derive_master_key, derive_subkey, encrypt, generate_salt,
};

#[test]
fn generated_keys_are_32_bytes() {
    let key = SecretKey::generate();

    assert_eq!(key.as_bytes().len(), 32);
}

#[test]
fn password_derivation_is_deterministic_for_same_inputs() {
    let salt = generate_salt();
    let password = b"test-password";

    let first = derive_master_key(password, &salt).unwrap();
    let second = derive_master_key(password, &salt).unwrap();

    assert_eq!(first.as_bytes(), second.as_bytes());
}

#[test]
fn different_salts_produce_different_keys() {
    let first_salt = generate_salt();
    let second_salt = generate_salt();
    let password = b"test-password";

    let first = derive_master_key(password, &first_salt).unwrap();
    let second = derive_master_key(password, &second_salt).unwrap();

    assert_ne!(first.as_bytes(), second.as_bytes());
}

#[test]
fn encryption_round_trip_works() {
    let key = SecretKey::generate();
    let plaintext = b"orvpass-secret";

    let encrypted = encrypt(&key, plaintext).unwrap();
    let decrypted = decrypt(&key, &encrypted).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn encryption_uses_unique_nonces() {
    let key = SecretKey::generate();
    let plaintext = b"same plaintext";

    let first = encrypt(&key, plaintext).unwrap();
    let second = encrypt(&key, plaintext).unwrap();

    assert_ne!(first.nonce, second.nonce);
    assert_ne!(first.ciphertext, second.ciphertext);
}

#[test]
fn wrong_key_cannot_decrypt() {
    let first_key = SecretKey::generate();
    let second_key = SecretKey::generate();

    let encrypted = encrypt(&first_key, b"secret").unwrap();

    assert!(decrypt(&second_key, &encrypted).is_err());
}

#[test]
fn tampering_is_detected() {
    let key = SecretKey::generate();
    let mut encrypted = encrypt(&key, b"secret").unwrap();

    encrypted.ciphertext[0] ^= 1;

    assert!(decrypt(&key, &encrypted).is_err());
}

#[test]
fn subkeys_are_context_separated() {
    let master = SecretKey::generate();

    let vault = derive_subkey(&master, b"vault").unwrap();
    let search = derive_subkey(&master, b"search").unwrap();

    assert_ne!(vault.as_bytes(), search.as_bytes());
}
