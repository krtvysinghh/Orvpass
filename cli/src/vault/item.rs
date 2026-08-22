use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct VaultItem {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub created: u128,
}

impl VaultItem {
    pub fn new(name: String, username: String, password: String, url: String) -> Self {
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Self {
            id: created.to_string(),
            name,
            username,
            password,
            url,
            created,
        }
    }

    pub fn serialize(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.id, self.name, self.username, self.password, self.url, self.created
        )
    }

    pub fn deserialize(line: &str) -> Option<Self> {
        let p: Vec<&str> = line.split('|').collect();

        if p.len() != 6 {
            return None;
        }

        Some(Self {
            id: p[0].into(),
            name: p[1].into(),
            username: p[2].into(),
            password: p[3].into(),
            url: p[4].into(),
            created: p[5].parse().unwrap_or(0),
        })
    }
}
