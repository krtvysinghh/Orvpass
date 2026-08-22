use zeroize::Zeroize;

pub struct SecureMemory {
    data: Vec<u8>,
}

impl SecureMemory {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn expose(&self) -> &[u8] {
        &self.data
    }
}

impl Drop for SecureMemory {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}
