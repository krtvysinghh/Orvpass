use crate::models::{Folder, VaultItem};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct VaultDatabase {
    items: Vec<VaultItem>,
    folders: Vec<Folder>,
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

    pub fn folder_count(&self) -> usize {
        self.folders.len()
    }

    pub fn insert(&mut self, item: VaultItem) {
        self.items.push(item);
    }

    pub fn get(&self, id: Uuid) -> Option<&VaultItem> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut VaultItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    pub fn remove(&mut self, id: Uuid) -> Option<VaultItem> {
        let index = self.items.iter().position(|item| item.id == id)?;
        Some(self.items.remove(index))
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    pub fn items(&self) -> &[VaultItem] {
        &self.items
    }
}
