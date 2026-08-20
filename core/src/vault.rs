use crate::{
    database::VaultDatabase,
    models::{ItemData, ItemType, VaultItem},
};

#[derive(Debug, Default)]
pub struct Vault {
    database: VaultDatabase,
    locked: bool,
}

impl Vault {
    pub fn new_locked() -> Self {
        Self {
            database: VaultDatabase::new(),
            locked: true,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn len(&self) -> usize {
        self.database.len()
    }

    pub fn is_empty(&self) -> bool {
        self.database.is_empty()
    }

    pub fn insert(&mut self, item: VaultItem) {
        self.database.insert(item);
    }

    pub fn create_login(&mut self, title: impl Into<String>) -> uuid::Uuid {
        let item = VaultItem::new(
            ItemType::Login,
            title,
            ItemData::Login(crate::models::LoginData {
                username: None,
                password: None,
                urls: Vec::new(),
            }),
        );

        let id = item.id;
        self.database.insert(item);
        id
    }

    pub fn item(&self, id: uuid::Uuid) -> Option<&VaultItem> {
        self.database.get(id)
    }

    pub fn items(&self) -> &[VaultItem] {
        self.database.items()
    }
}
