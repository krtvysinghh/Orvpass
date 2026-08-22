use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct SyncPacket {
    pub device_id: String,

    pub timestamp: u64,

    pub payload: Vec<u8>,
}

pub fn create_packet(device: String, data: Vec<u8>) -> SyncPacket {
    SyncPacket {
        device_id: device,

        timestamp: 0,

        payload: data,
    }
}
