use zeroize::Zeroize;

pub struct SecureBuffer(Vec<u8>);

impl SecureBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for SecureBuffer {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}
