pub mod conflict;
pub mod device;
pub mod protocol;
pub mod storage;

#[derive(Debug, Clone)]
pub enum SyncMode {
    Local,
    Lan,
    Cloud,
}

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub mode: SyncMode,

    pub encrypted: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            mode: SyncMode::Local,

            encrypted: true,
        }
    }
}
