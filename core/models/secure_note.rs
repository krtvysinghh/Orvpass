use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureNote {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
