use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportPackage {
    pub version: String,
    pub created: String,
    pub items: Vec<serde_json::Value>,
}
