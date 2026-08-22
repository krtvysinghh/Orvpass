use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {

    pub id:String,
    pub name:String,
    pub last_sync:u64,

}
