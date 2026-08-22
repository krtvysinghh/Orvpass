use crate::vault::database;
use crate::vault::database::Item;

pub fn list() {
    let items = database::list();

    for item in items {
        println!("{} | {}", item.name, item.username);
    }
}

pub fn add() {
    let item = Item {
        name: "example".into(),
        username: "user".into(),
        password: "password".into(),
    };

    database::add(item);

    println!("Item added");
}

pub fn remove(name: String) {
    database::remove(&name);

    println!("Removed {}", name);
}
