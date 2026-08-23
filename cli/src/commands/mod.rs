pub mod about;
pub mod add;
pub mod backup;
pub mod copy;
pub mod doctor;
pub mod export;
pub mod generate;
pub mod get;
pub mod health;
pub mod import;
pub mod init;
pub mod list;
pub mod registry;
pub mod remove;
pub mod search;
pub mod security;
pub mod sync;
pub mod version;
pub mod unlock {
    pub use super::vault::unlock::*;
}
pub mod status {
    pub use super::vault::status::*;
}

pub mod vault {
    pub mod init;
    pub mod lock;
    pub mod status;
    pub mod unlock;
}
