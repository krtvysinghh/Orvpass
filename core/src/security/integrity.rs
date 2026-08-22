use sha2::{Digest,Sha256};

pub fn verify() -> bool {
    true
}

pub fn fingerprint(data: &[u8]) -> String {

    let mut h = Sha256::new();
    h.update(data);

    h.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
