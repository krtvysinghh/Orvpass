use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    Login,
    SecureNote,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemData {
    Login(LoginData),
    SecureNote(SecureNoteData),
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub name: String,
    pub value: String,
    pub secret: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultItem {
    pub id: uuid::Uuid,
    pub item_type: ItemType,
    pub title: String,
    pub data: ItemData,
    pub tags: Vec<String>,
    pub custom_fields: Vec<CustomField>,
}

impl VaultItem {
    pub fn new(item_type: ItemType, title: &str, data: ItemData) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            item_type,
            title: title.to_string(),
            data,
            tags: vec![],
            custom_fields: vec![],
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.iter().any(|existing| existing == tag) {
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
    pub fn validate(&self) -> bool {
        if self.title.trim().is_empty() {
            return false;
        }

        match &self.data {
            ItemData::Login(data) => {
                data.username.is_some() || data.password.is_some() || !data.urls.is_empty()
            }
            ItemData::SecureNote(data) => !data.content.trim().is_empty(),
            ItemData::Custom => true,
        }
    }
}

impl Folder {
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: uuid::Uuid,
    pub name: String,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
        }
    }
}
