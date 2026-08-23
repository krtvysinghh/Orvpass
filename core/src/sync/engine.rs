use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDevice {
    pub id: Uuid,
    pub name: String,
    pub last_seen: i64,
}

impl SyncDevice {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            last_seen: Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncAction {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub id: Uuid,
    pub device: Uuid,
    pub action: SyncAction,
    pub object_id: Uuid,
    pub timestamp: i64,
}

impl SyncEvent {
    pub fn new(device: Uuid, action: SyncAction, object_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            device,
            action,
            object_id,
            timestamp: Utc::now().timestamp(),
        }
    }
}

pub struct SyncEngine {
    pub device: SyncDevice,
    pub queue: Vec<SyncEvent>,
}

impl SyncEngine {
    pub fn new(name: &str) -> Self {
        Self {
            device: SyncDevice::new(name),
            queue: Vec::new(),
        }
    }

    pub fn push(&mut self, event: SyncEvent) {
        self.queue.push(event);
    }

    pub fn pending(&self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
