use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct VaultEntry {
    pub id:String,
    pub name:String,
    pub username:String,
    pub url:String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AppInfo {
    pub name:String,
    pub version:String,
}
