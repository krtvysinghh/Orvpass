use crate::{
    crypto::{CryptoError, EncryptedData, SecretKey, decrypt, encrypt},
    database::VaultDatabase,
    models::{ItemData, ItemType, VaultItem},
};
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const FILE_MAGIC: &[u8; 8] = b"ORVLT001";
const FILE_VERSION: u16 = 1;

#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("vault is locked")]
    Locked,

    #[error("vault file does not exist")]
    NotFound,

    #[error("invalid vault file")]
    InvalidFile,

    #[error("unsupported vault version")]
    UnsupportedVersion,

    #[error("cryptographic failure: {0}")]
    Crypto(#[from] CryptoError),

    #[error("serialization failure")]
    Serialization,

    #[error("filesystem failure: {0}")]
    Io(#[from] io::Error),

    #[error("vault is temporarily locked after too many failed unlock attempts")]
    TemporarilyLocked,

    #[error("unlock failed")]
    UnlockFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultState {
    Locked,
    Unlocked,
    TemporarilyLocked,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VaultStatus {
    pub state: VaultState,
    pub failed_attempts: u32,
    pub max_failed_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistedVault {
    items: Vec<VaultItem>,
}

#[derive(Debug)]
pub struct Vault {
    database: VaultDatabase,
    locked: bool,
    path: Option<PathBuf>,
    policy: LockPolicy,
    failed_attempts: u32,
    lockout_until: Option<Instant>,
    unlocked_at: Option<SystemTime>,
}

impl Default for Vault {
    fn default() -> Self {
        Self::new_locked()
    }
}

impl Vault {
    pub fn new_locked() -> Self {
        Self {
            database: VaultDatabase::new(),
            locked: true,
            path: None,
            policy: LockPolicy::default(),
            failed_attempts: 0,
            lockout_until: None,
            unlocked_at: None,
        }
    }

    pub fn new_locked_at(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Some(path.into()),
            ..Self::new_locked()
        }
    }

    pub fn with_policy(mut self, policy: LockPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn state(&self) -> VaultState {
        if self.lockout_active() {
            VaultState::TemporarilyLocked
        } else if self.locked {
            VaultState::Locked
        } else {
            VaultState::Unlocked
        }
    }

    pub fn status(&self) -> VaultStatus {
        VaultStatus {
            state: self.state(),
            failed_attempts: self.failed_attempts,
            max_failed_attempts: self.policy.max_failed_attempts,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.state() != VaultState::Unlocked
    }

    pub fn is_unlocked(&self) -> bool {
        self.state() == VaultState::Unlocked
    }

    pub fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    pub fn remaining_lockout(&self) -> Option<Duration> {
        self.lockout_until
            .and_then(|until| until.checked_duration_since(Instant::now()))
    }

    pub fn unlocked_at(&self) -> Option<SystemTime> {
        self.unlocked_at
    }

    pub fn len(&self) -> usize {
        self.database.len()
    }

    pub fn is_empty(&self) -> bool {
        self.database.is_empty()
    }

    pub fn insert(&mut self, item: VaultItem) -> Result<(), VaultError> {
        self.require_unlocked()?;
        self.database.insert(item);
        Ok(())
    }

    pub fn create_login(&mut self, title: impl Into<String>) -> Result<Uuid, VaultError> {
        self.require_unlocked()?;

        let item = VaultItem::new(
            ItemType::Login,
            title,
            ItemData::Login(crate::models::LoginData {
                username: None,
                password: None,
                urls: Vec::new(),
            }),
        );

        let id = item.id;
        self.database.insert(item);

        Ok(id)
    }

    pub fn item(&self, id: Uuid) -> Result<Option<&VaultItem>, VaultError> {
        self.require_unlocked()?;
        Ok(self.database.get(id))
    }

    pub fn items(&self) -> Result<&[VaultItem], VaultError> {
        self.require_unlocked()?;
        Ok(self.database.items())
    }

    pub fn initialize(&mut self, key: &SecretKey) -> Result<(), VaultError> {
        if self.path.is_none() {
            return Err(VaultError::NotFound);
        }

        self.database = VaultDatabase::new();
        self.locked = false;
        self.failed_attempts = 0;
        self.lockout_until = None;
        self.unlocked_at = Some(SystemTime::now());

        self.save(key)
    }

    pub fn unlock(&mut self, key: &SecretKey) -> Result<(), VaultError> {
        if self.lockout_active() {
            return Err(VaultError::TemporarilyLocked);
        }

        let path = self.path.clone().ok_or(VaultError::NotFound)?;
        let bytes = fs::read(path)?;

        match decode_and_decrypt(&bytes, key) {
            Ok(persisted) => {
                let mut database = VaultDatabase::new();

                for item in persisted.items {
                    database.insert(item);
                }

                self.database = database;
                self.locked = false;
                self.failed_attempts = 0;
                self.lockout_until = None;
                self.unlocked_at = Some(SystemTime::now());

                Ok(())
            }
            Err(VaultError::Crypto(_)) | Err(VaultError::Serialization) => {
                self.register_failed_unlock();
                Err(VaultError::UnlockFailed)
            }
            Err(error) => Err(error),
        }
    }

    pub fn save(&self, key: &SecretKey) -> Result<(), VaultError> {
        self.require_unlocked()?;

        let path = self.path.as_deref().ok_or(VaultError::NotFound)?;

        let persisted = PersistedVault {
            items: self.database.items().to_vec(),
        };

        let plaintext = postcard::to_allocvec(&persisted).map_err(|_| VaultError::Serialization)?;

        let encrypted = encrypt(key, &plaintext)?;

        let mut output = Vec::with_capacity(
            FILE_MAGIC.len()
                + std::mem::size_of::<u16>()
                + encrypted.nonce.len()
                + encrypted.ciphertext.len(),
        );

        output.extend_from_slice(FILE_MAGIC);
        output.extend_from_slice(&FILE_VERSION.to_le_bytes());
        output.extend_from_slice(&encrypted.nonce);
        output.extend_from_slice(&encrypted.ciphertext);

        atomic_write(path, &output)?;

        Ok(())
    }

    pub fn lock(&mut self) {
        self.database = VaultDatabase::new();
        self.locked = true;
        self.unlocked_at = None;
    }

    fn register_failed_unlock(&mut self) {
        self.database = VaultDatabase::new();
        self.locked = true;
        self.unlocked_at = None;

        self.failed_attempts = self.failed_attempts.saturating_add(1);

        if self.failed_attempts >= self.policy.max_failed_attempts {
            self.lockout_until =
                Some(Instant::now() + Duration::from_secs(self.policy.lockout_seconds));
        }
    }

    fn lockout_active(&self) -> bool {
        matches!(
            self.lockout_until,
            Some(until) if until > Instant::now()
        )
    }

    fn require_unlocked(&self) -> Result<(), VaultError> {
        if self.lockout_active() {
            return Err(VaultError::TemporarilyLocked);
        }

        if self.locked {
            return Err(VaultError::Locked);
        }

        Ok(())
    }
}

fn decode_and_decrypt(bytes: &[u8], key: &SecretKey) -> Result<PersistedVault, VaultError> {
    const HEADER: usize = 8 + 2 + 12;

    if bytes.len() < HEADER + 16 {
        return Err(VaultError::InvalidFile);
    }

    if &bytes[..8] != FILE_MAGIC {
        return Err(VaultError::InvalidFile);
    }

    let version = u16::from_le_bytes([bytes[8], bytes[9]]);

    if version != FILE_VERSION {
        return Err(VaultError::UnsupportedVersion);
    }

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&bytes[10..22]);

    let encrypted = EncryptedData {
        nonce,
        ciphertext: bytes[22..].to_vec(),
    };

    let plaintext = decrypt(key, &encrypted)?;

    postcard::from_bytes(&plaintext).map_err(|_| VaultError::Serialization)
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<(), io::Error> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;

    let temp_name = format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("vault"),
        Uuid::new_v4()
    );

    let temp_path = parent.join(temp_name);

    fs::write(&temp_path, data)?;

    if let Ok(file) = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&temp_path)
    {
        file.sync_all()?;
    }

    fs::rename(&temp_path, path)?;

    if let Ok(dir) = fs::File::open(parent) {
        let _ = dir.sync_all();
    }

    Ok(())
}

#[allow(dead_code)]
fn unix_timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
