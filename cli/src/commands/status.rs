use crate::vault;

pub fn execute() {
    if vault::path().exists() {
        println!("Vault ready");
    } else {
        println!("Vault missing");
    }
}
