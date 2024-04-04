#[derive(Debug, Clone)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: String, age: u8, id: usize) -> Self {
        Self { name, age, id }
    }
}
