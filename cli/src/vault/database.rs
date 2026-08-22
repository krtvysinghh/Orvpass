use std::fs;
use std::path::PathBuf;

use super::encryption;

fn path() -> PathBuf {
    dirs::home_dir().unwrap().join(".orvpass").join("vault.enc")
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl Item {
    fn serialize(&self) -> String {
        format!("{}|{}|{}", self.name, self.username, self.password)
    }

    fn deserialize(s: &str) -> Option<Self> {
        let p: Vec<&str> = s.split('|').collect();

        if p.len() != 3 {
            return None;
        }

        Some(Self {
            name: p[0].into(),
            username: p[1].into(),
            password: p[2].into(),
        })
    }
}

pub fn list() -> Vec<Item> {
    let data = load();

    data.lines().filter_map(Item::deserialize).collect()
}

pub fn add(item: Item) {
    let mut items = list();

    items.push(item);

    save(items);
}

pub fn remove(name: &str) {
    let items: Vec<Item> = list().into_iter().filter(|x| x.name != name).collect();

    save(items);
}

fn save(items: Vec<Item>) {
    let raw = items
        .iter()
        .map(|x| x.serialize())
        .collect::<Vec<_>>()
        .join("\n");

    let encrypted = encryption::encrypt(&raw, "master");

    let p = path();

    fs::create_dir_all(p.parent().unwrap()).unwrap();

    fs::write(p, encrypted).unwrap();
}

fn load() -> String {
    let p = path();

    if !p.exists() {
        return String::new();
    }

    let data = fs::read_to_string(p).unwrap();

    encryption::decrypt(&data, "master").unwrap_or_default()
}
