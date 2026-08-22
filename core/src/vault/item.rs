use serde::{Serialize,Deserialize};
use uuid::Uuid;

use crate::models::{ItemData,ItemType};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct VaultItem {
    pub id:String,
    pub item_type:String,
    pub name:String,
    pub data:String,
}

impl VaultItem {
    pub fn new(
        item_type:ItemType,
        name:&str,
        data:ItemData,
    )->Self {
        Self{
            id:Uuid::new_v4().to_string(),
            item_type:format!("{:?}",item_type),
            name:name.to_string(),
            data:format!("{:?}",data),
        }
    }
}
