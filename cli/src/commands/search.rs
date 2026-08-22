use crate::vault::database;

pub fn execute(query: &str) {
    let items = database::list();

    let mut found = false;

    for item in items {
        if item.name.contains(query) {
            println!("{}", item.name);
            found = true;
        }
    }

    if !found {
        println!("No matches");
    }
}
