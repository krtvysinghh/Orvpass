use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedItem {
    pub name: String,

    pub username: Option<String>,

    pub password: Option<String>,

    pub url: Option<String>,

    pub notes: Option<String>,
}

pub trait Importer {
    fn import(data: &str) -> Result<Vec<ImportedItem>, String>;
}
pub mod onepassword;
pub mod lastpass;
pub mod chrome;
pub mod apple;
pub mod proton;
pub mod dashlane;
