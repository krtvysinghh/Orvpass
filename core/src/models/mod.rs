use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Login,
    SecureNote,
    Custom,
    Totp,
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
pub enum ItemData {
    Login(LoginData),
    SecureNote(SecureNoteData),
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

    pub fn new(
        item_type: ItemType,
        name: &str,
        data: ItemData,
    ) -> Self {
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

    pub fn add_custom_field(
        &mut self,
        name: &str,
        value: &str,
        secret: bool,
    ) {
        self.custom_fields.push(CustomField {
            name: name.to_string(),
            value: value.to_string(),
            secret,
        });
    }
}
