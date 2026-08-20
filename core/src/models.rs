use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TotpData {
    pub secret: String,
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub algorithm: String,
    pub digits: u8,
    pub period_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecureNoteData {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CardData {
    pub cardholder_name: Option<String>,
    pub number: Option<String>,
    pub expiry_month: Option<u8>,
    pub expiry_year: Option<u16>,
    pub security_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityData {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomField {
    pub name: String,
    pub value: String,
    pub secret: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemData {
    Login(LoginData),
    Passkey,
    Totp(TotpData),
    SecureNote(SecureNoteData),
    CreditCard(CardData),
    Identity(IdentityData),
    BankAccount,
    ApiKey,
    SshKey,
    RecoveryCodes(Vec<String>),
    Wifi,
    SoftwareLicense,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VaultItem {
    pub id: Uuid,
    pub item_type: ItemType,
    pub title: String,
    pub username: Option<String>,
    pub favorite: bool,
    pub archived: bool,
    pub tags: Vec<String>,
    pub folder_id: Option<Uuid>,
    pub custom_fields: Vec<CustomField>,
    pub data: ItemData,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VaultItem {
    pub fn new(item_type: ItemType, title: impl Into<String>, data: ItemData) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            item_type,
            title: title.into(),
            username: None,
            favorite: false,
            archived: false,
            tags: Vec::new(),
            folder_id: None,
            custom_fields: Vec::new(),
            data,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();

        if !self.tags.iter().any(|existing| existing == &tag) {
            self.tags.push(tag);
            self.touch();
        }
    }

    pub fn add_custom_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        secret: bool,
    ) {
        self.custom_fields.push(CustomField {
            name: name.into(),
            value: value.into(),
            secret,
        });
        self.touch();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Folder {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }
}
