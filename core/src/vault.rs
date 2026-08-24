use crate::crypto::{SecretKey, decrypt, derive_master_key, encrypt};
use crate::models::{ItemData, ItemType, LoginData, VaultItem};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const VAULT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultState {
    Locked,
    Unlocked,
    TemporarilyLocked,
}

#[derive(Debug, Clone, Copy)]
pub struct LockPolicy {
    pub max_failed_attempts: u32,
    pub lockout_seconds: u64,
}

impl Default for LockPolicy {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_seconds: 30,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct VaultPayload {
    version: u32,
    salt: [u8; 16],
    items: Vec<VaultItem>,
}

#[derive(Debug)]
pub enum VaultError {
    Locked,
    UnlockFailed,
    TemporarilyLocked,
    Invalid,
}

pub struct Vault {
    path: PathBuf,
    unlocked: bool,
    key_check: Option<[u8; 32]>,
    salt: Option<[u8; 16]>,
    items: Vec<VaultItem>,
    failed_attempts: u32,
    policy: LockPolicy,
    locked_until: Option<SystemTime>,
    unlocked_at: Option<SystemTime>,
}

impl Vault {
    pub fn items(&self) -> &[VaultItem] {
        &self.items
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn new(path: PathBuf) -> Self {
        Self::new_locked_at(&path)
    }
    pub fn new_locked() -> Self {
        Self {
            path: PathBuf::new(),
            unlocked: false,
            key_check: None,
            salt: None,
            items: Vec::new(),
            failed_attempts: 0,
            policy: LockPolicy::default(),
            locked_until: None,
            unlocked_at: None,
        }
    }

    pub fn new_locked_at(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            unlocked: false,
            key_check: None,
            salt: None,
            items: Vec::new(),
            failed_attempts: 0,
            policy: LockPolicy::default(),
            locked_until: None,
            unlocked_at: None,
        }
    }

    pub fn initialize(&mut self, key: &SecretKey) -> Result<(), VaultError> {
        self.key_check = Some(*key.as_bytes());
        self.unlocked = true;
        self.save(key)
    }

    pub fn unlock(&mut self, key: &SecretKey) -> Result<(), VaultError> {
        if self.locked_until.is_some() {
            return Err(VaultError::TemporarilyLocked);
        }

        if self.path.exists() {
            let raw = fs::read(&self.path).map_err(|_| VaultError::Invalid)?;

            if raw.len() < 12 {
                return Err(VaultError::Invalid);
            }

            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&raw[..12]);

            let encrypted = crate::crypto::EncryptedData {
                nonce,
                ciphertext: raw[12..].to_vec(),
            };

            let decrypted = match decrypt(key, &encrypted) {
                Ok(data) => data,
                Err(_) => {
                    self.failed_attempts += 1;

                    if self.failed_attempts >= self.policy.max_failed_attempts {
                        self.locked_until = Some(
                            SystemTime::now()
                                + std::time::Duration::from_secs(self.policy.lockout_seconds),
                        );
                    }

                    return Err(VaultError::UnlockFailed);
                }
            };

            let payload: VaultPayload =
                postcard::from_bytes(&decrypted).map_err(|_| VaultError::Invalid)?;

            if payload.version != VAULT_VERSION {
                return Err(VaultError::Invalid);
            }

            self.salt = Some(payload.salt);
            self.items = payload.items;
        } else if self
            .key_check
            .is_some_and(|expected| expected != *key.as_bytes())
        {
            return Err(VaultError::UnlockFailed);
        }

        self.unlocked = true;
        self.failed_attempts = 0;
        self.locked_until = None;
        self.unlocked_at = Some(SystemTime::now());

        Ok(())
    }

    pub fn unlock_with_password(&mut self, password: &str) -> Result<(), VaultError> {
        let salt = self.salt.ok_or(VaultError::Invalid)?;

        let key =
            derive_master_key(password.as_bytes(), &salt).map_err(|_| VaultError::UnlockFailed)?;

        self.unlock(&key)
    }

    pub fn lock(&mut self) {
        self.unlocked = false;
        self.unlocked_at = None;
    }

    pub fn is_locked(&self) -> bool {
        !self.unlocked
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    pub fn state(&self) -> VaultState {
        if self.locked_until.is_some() {
            return VaultState::TemporarilyLocked;
        }

        if self.unlocked {
            VaultState::Unlocked
        } else {
            VaultState::Locked
        }
    }

    pub fn remaining_lockout(&self) -> Option<std::time::Duration> {
        self.locked_until
            .and_then(|until| until.duration_since(SystemTime::now()).ok())
    }

    pub fn unlocked_at(&self) -> Option<SystemTime> {
        self.unlocked_at
    }

    pub fn with_policy(mut self, policy: LockPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn insert(&mut self, item: VaultItem) -> Result<(), VaultError> {
        if !self.unlocked {
            return Err(VaultError::Locked);
        }

        self.items.push(item);
        Ok(())
    }

    pub fn create_login(&mut self, name: &str) -> Result<String, VaultError> {
        if !self.unlocked {
            return Err(VaultError::Locked);
        }

        let item = VaultItem::new(
            ItemType::Login,
            name,
            ItemData::Login(LoginData {
                username: None,
                password: None,
                urls: vec![],
            }),
        );

        let id = item.id;
        self.items.push(item);

        Ok(id.to_string())
    }

    pub fn delete(&mut self, id: String) -> Result<(), VaultError> {
        self.items.retain(|x| x.id.to_string() != id);
        Ok(())
    }

    pub fn item<T: ToString>(&self, id: T) -> Result<Option<VaultItem>, VaultError> {
        let id = id.to_string();

        Ok(self
            .items
            .iter()
            .find(|item| item.id.to_string() == id)
            .cloned())
    }

    pub fn save(&self, key: &SecretKey) -> Result<(), VaultError> {
        let payload = VaultPayload {
            version: 1,
            salt: self.salt.unwrap_or([0u8; 16]),
            items: self.items.clone(),
        };

        let serialized = postcard::to_allocvec(&payload).map_err(|_| VaultError::Invalid)?;

        let encrypted = encrypt(key, &serialized).map_err(|_| VaultError::Invalid)?;

        let mut output = Vec::new();

        output.extend_from_slice(&encrypted.nonce);
        output.extend_from_slice(&encrypted.ciphertext);

        let tmp = self.path.with_extension("tmp");

        fs::write(&tmp, output).map_err(|_| VaultError::Invalid)?;

        fs::rename(&tmp, &self.path).map_err(|_| VaultError::Invalid)?;

        Ok(())
    }

    pub fn vault_version(&self) -> u32 {
        VAULT_VERSION
    }

    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::Locked => write!(f, "vault locked"),
            VaultError::UnlockFailed => write!(f, "unlock failed"),
            VaultError::TemporarilyLocked => write!(f, "temporarily locked"),
            VaultError::Invalid => write!(f, "invalid vault"),
        }
    }
}

impl std::error::Error for VaultError {}
