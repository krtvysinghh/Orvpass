use crate::vault;
use crate::vault::session;

pub fn execute(query: &str) {
    let Some(key) = session::key() else {
        println!("Vault locked");
        return;
    };

    let mut vault = vault::open();

    if vault.unlock(&key).is_err() {
        println!("Unlock failed");
        return;
    }

    for item in vault.items() {
        if item.name.to_lowercase().contains(&query.to_lowercase()) {
            println!("{}", item.name);
        }
    }
}
