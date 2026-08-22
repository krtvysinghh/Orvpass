use crate::vault;
use crate::vault::session;

pub fn execute() {
    let Some(key) = session::key() else {
        println!("Vault locked");
        return;
    };

    let mut v = vault::open();

    if v.unlock(&key).is_err() {
        println!("Unlock failed");
        return;
    }

    for item in v.items() {
        println!("{}", item.name);
    }
}
