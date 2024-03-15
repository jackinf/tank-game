use crate::common::describable::Describable;
use crate::todo::todo_item::TodoItem;

// #[derive(Debug, Default)]
pub struct TodoManager {
    id_counter: i32,
    items: Vec<TodoItem>,
}

impl TodoManager {
    pub fn add_item(&mut self, title: String) -> i32 {
        let id = self.id_counter;
        let item = TodoItem {
            id,
            title,
            is_done: false,
        };
        self.items.push(item);
        self.id_counter += 1;
        id
    }

    pub fn list_items(&self) -> () {
        self.items.iter().for_each(|item| {
            println!(
                "id: {}, title: {}, is_done: {}",
                item.id, item.title, item.is_done
            );
        });
    }

    pub fn update_item(&mut self, id: i32, title: String) {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            item.title = title;
        } else {
            println!("Item was not found");
        }
    }

    pub fn remove_item(&mut self, id: i32) {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(index);
        } else {
            println!("Item was not found");
        }
    }
}

impl Default for TodoManager {
    fn default() -> Self {
        println!("default called");
        Self {
            id_counter: 1,
            items: Vec::new(),
        }
    }
}

impl Describable for TodoManager {
    fn describe(&self) -> String {
        let suffix = if self.items.len() != 1 { "s" } else { "" };
        String::from(format!("Manager has {} item{}", self.items.len(), suffix))
    }
}
