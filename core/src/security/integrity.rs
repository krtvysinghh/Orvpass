use sha2::{Digest, Sha256};

pub fn fingerprint(data: &[u8]) -> String {
    let mut h = Sha256::new();

    h.update(data);

    hex::encode(h.finalize())
}
