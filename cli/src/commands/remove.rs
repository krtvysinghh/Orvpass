use crate::vault::database;

pub fn execute(name: String) {
    database::remove(&name);

    println!("Removed {}", name);
}
