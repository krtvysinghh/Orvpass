use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPacket {

    pub device_id: String,
    pub vault_id: String,
    pub version: u64,
    pub timestamp: u64,
    pub encrypted_payload: Vec<u8>,

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncAction {

    Create,
    Update,
    Delete,

}
