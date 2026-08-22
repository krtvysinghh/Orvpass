use crate::vault;
use crate::vault::session;

pub fn execute(name: String) {
    let Some(key) = session::key() else {
        println!("Vault locked");
        return;
    };

    let mut v = vault::open();

    if v.unlock(&key).is_err() {
        println!("Unlock failed");
        return;
    }

    if v.delete(name.clone()).is_ok() {
        let _ = v.save(&key);
        println!("Removed {}", name);
    }
}
