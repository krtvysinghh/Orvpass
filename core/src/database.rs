use crate::models::VaultItem;

#[derive(Debug, Default)]
pub struct VaultDatabase {
    items: Vec<VaultItem>,
}

impl VaultDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
