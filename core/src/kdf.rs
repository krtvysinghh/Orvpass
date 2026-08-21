use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2,
};

pub fn derive_key(password: &str, salt: &[u8; 16]) -> [u8; 32] {
    let salt = SaltString::encode_b64(salt).unwrap();

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let parsed = PasswordHash::new(&hash).unwrap();

    let bytes = parsed.hash.unwrap();

    let mut out = [0u8; 32];

    out.copy_from_slice(&bytes.as_bytes()[..32]);

    out
}
