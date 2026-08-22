use sha2::{Digest, Sha256};

pub fn create(data: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(data);

    hex::encode(hash.finalize())
}
