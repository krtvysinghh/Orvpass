use crate::vault::database;

pub fn execute(name: &str) {
    for item in database::list() {
        if item.name == name {
            println!("Name: {}", item.name);
            println!("Username: {}", item.username);
            println!("Password: {}", item.password);

            return;
        }
    }

    println!("Not found");
}
