use argon2::{
    Argon2,
    password_hash::{
        PasswordHasher,
        SaltString
    }
};

pub fn derive_key(password: &str) -> String {

    let salt = SaltString::encode_b64(
        b"orvpass-static-salt"
    ).unwrap();

    let argon = Argon2::default();

    let hash = argon
        .hash_password(password.as_bytes(), &salt)
        .unwrap();

    hash.to_string()
}
