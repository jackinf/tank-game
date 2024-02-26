mod common {
    pub mod describable;
}
mod todo {
    pub mod todo_item;
    pub mod todo_manager;
}
mod star_wars {
    pub mod api;
    pub mod person;
}

use crate::common::describable::Describable;
use crate::star_wars::api::get_person;
use crate::todo::todo_manager::{TodoManager};

fn print_description<T: Describable>(item: T) {
    println!("{}", item.describe())
}

#[tokio::main]
async fn main() {
    // using Task manager
    let mut manager = TodoManager::default();
    let id1 = manager.add_item(String::from("Some Item 1"));
    let id2 = manager.add_item(String::from("Some Item 2"));
    manager.add_item(String::from("Some Item 3"));
    manager.list_items();
    manager.update_item(id1, String::from("Some Item 1 (updated)"));
    manager.remove_item(id2);
    manager.list_items();

    print_description(manager);

    // Star wars
    match get_person(1).await {
        Ok(person) => print_description(person),
        Err(e) => println!("Error: {}", e),
    }
}
