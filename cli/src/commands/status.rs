use crate::vault::database;

pub fn execute() {
    let count = database::list().len();

    println!("Vault entries: {}", count);
}
