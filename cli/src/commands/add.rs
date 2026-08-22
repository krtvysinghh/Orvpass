use crate::vault::database;
use crate::vault::database::Item;

pub fn execute(name: String) {
    let item = Item {
        name,
        username: String::new(),
        password: String::new(),
    };

    database::add(item);

    println!("Item added");
}
