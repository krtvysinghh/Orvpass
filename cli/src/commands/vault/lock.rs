use crate::vault::session;

pub fn execute() {
    session::lock();

    println!("Vault locked");
}
