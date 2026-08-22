use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit},
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const MASTER_KEY_BYTES: usize = 32;
pub const NONCE_BYTES: usize = 12;
pub const SALT_BYTES: usize = 16;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("invalid cryptographic parameters")]
    InvalidParameters,

    #[error("key derivation failed")]
    KeyDerivation,

    #[error("encryption failed")]
    Encryption,

    #[error("decryption failed")]
    Decryption,

    #[error("invalid ciphertext")]
    InvalidCiphertext,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop, Debug, PartialEq, Eq)]
pub struct SecretKey([u8; MASTER_KEY_BYTES]);

impl SecretKey {
    pub fn from_password(password: &str) -> Result<Self, CryptoError> {
        let salt = [0u8; SALT_BYTES];

        derive_master_key(password.as_bytes(), &salt)
    }

    pub fn from_bytes(bytes: [u8; MASTER_KEY_BYTES]) -> Self {
        Self(bytes)
    }

    pub fn generate() -> Self {
        let mut bytes = [0u8; MASTER_KEY_BYTES];
        rand::rng().fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; MASTER_KEY_BYTES] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedData {
    pub nonce: [u8; NONCE_BYTES],
    pub ciphertext: Vec<u8>,
}

pub fn generate_salt() -> [u8; SALT_BYTES] {
    let mut salt = [0u8; SALT_BYTES];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_master_key(
    password: &[u8],
    salt: &[u8; SALT_BYTES],
) -> Result<SecretKey, CryptoError> {
    let params = Params::new(64 * 1024, 3, 4, Some(MASTER_KEY_BYTES))
        .map_err(|_| CryptoError::InvalidParameters)?;

    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut output = [0u8; MASTER_KEY_BYTES];

    argon
        .hash_password_into(password, salt, &mut output)
        .map_err(|_| CryptoError::KeyDerivation)?;

    Ok(SecretKey::from_bytes(output))
}

pub fn derive_subkey(master_key: &SecretKey, context: &[u8]) -> Result<SecretKey, CryptoError> {
    let hk = Hkdf::<Sha256>::new(None, master_key.as_bytes());
    let mut output = [0u8; MASTER_KEY_BYTES];

    hk.expand(context, &mut output)
        .map_err(|_| CryptoError::KeyDerivation)?;

    Ok(SecretKey::from_bytes(output))
}

pub fn encrypt(key: &SecretKey, plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
    let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
        .map_err(|_| CryptoError::InvalidParameters)?;

    let mut nonce_bytes = [0u8; NONCE_BYTES];
    rand::rng().fill_bytes(&mut nonce_bytes);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
        .map_err(|_| CryptoError::Encryption)?;

    Ok(EncryptedData {
        nonce: nonce_bytes,
        ciphertext,
    })
}

pub fn decrypt(key: &SecretKey, encrypted: &EncryptedData) -> Result<Vec<u8>, CryptoError> {
    if encrypted.ciphertext.len() < 16 {
        return Err(CryptoError::InvalidCiphertext);
    }

    let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
        .map_err(|_| CryptoError::InvalidParameters)?;

    cipher
        .decrypt(
            Nonce::from_slice(&encrypted.nonce),
            encrypted.ciphertext.as_ref(),
        )
        .map_err(|_| CryptoError::Decryption)
}
