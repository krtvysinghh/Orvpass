use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemType {
    Login,
    Passkey,
    Totp,
    SecureNote,
    CreditCard,
    Identity,
    BankAccount,
    ApiKey,
    SshKey,
    RecoveryCodes,
    Wifi,
    SoftwareLicense,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultItem {
    pub id: Uuid,
    pub item_type: ItemType,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VaultItem {
    pub fn new(item_type: ItemType, title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            item_type,
            title: title.into(),
            created_at: now,
            updated_at: now,
        }
    }
}
