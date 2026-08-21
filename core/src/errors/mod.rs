use std::fmt;

#[derive(Debug)]
pub enum OrvpassError {
    InvalidPassword,
    VaultLocked,
    NotFound,
    StorageFailure,
    CryptoFailure,
}

impl fmt::Display for OrvpassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for OrvpassError {}
