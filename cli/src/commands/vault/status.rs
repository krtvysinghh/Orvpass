use crate::vault::session;

pub fn execute() {
    if session::is_unlocked() {
        println!("Vault: unlocked");
    } else {
        println!("Vault: locked");
    }
}
