use crate::vault;
use crate::vault::session;

pub fn execute(name: &str) {
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
        if item.name == name {
            println!("Name: {}", item.name);
            println!("{:?}", item.data);
            return;
        }
    }

    println!("Not found");
}
