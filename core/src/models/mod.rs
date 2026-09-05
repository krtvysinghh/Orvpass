use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Login,
    SecureNote,
    Custom,
    Totp,
    CreditCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureNoteData {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub name: String,
    pub value: String,
    pub secret: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardData {
    pub cardholder_name: String,
    pub card_number: String,
    pub expiration_month: String,
    pub expiration_year: String,
    pub cvv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemData {
    Login(LoginData),
    SecureNote(SecureNoteData),
    CreditCard(CreditCardData),
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultItem {
    pub id: Uuid,
    pub title: String,
    pub name: String,
    pub item_type: ItemType,
    pub data: ItemData,
    pub tags: Vec<String>,
    pub custom_fields: Vec<CustomField>,
}

impl VaultItem {
    pub fn new(item_type: ItemType, name: &str, data: ItemData) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: name.to_string(),
            name: name.to_string(),
            item_type,
            data,
            tags: Vec::new(),
            custom_fields: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }

    pub fn add_custom_field(&mut self, name: &str, value: &str, secret: bool) {
        self.custom_fields.push(CustomField {
            name: name.to_string(),
            value: value.to_string(),
            secret,
        });
    }
}

impl VaultItem {
    pub fn compute_item_hash(&self) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(self.title.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl VaultItem {
    pub fn sanitize_inputs(&mut self) {
        self.title = self.title.trim().chars().take(256).collect();
    }
}

impl VaultItem {
    pub fn validate_schema(&self) -> bool {
        self.title.len() <= 1024 && self.tags.len() <= 100
    }
}
