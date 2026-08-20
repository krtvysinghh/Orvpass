use crate::models::VaultItem;

#[derive(Debug, Default)]
pub struct Vault {
    items: Vec<VaultItem>,
    locked: bool,
}

impl Vault {
    pub fn new_locked() -> Self {
        Self {
            items: Vec::new(),
            locked: true,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
