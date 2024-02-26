use serde::Deserialize;
use crate::common::describable::Describable;

#[derive(Deserialize)]
pub struct Person {
    name: String,
    height: String,
    mass: String
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("name: {}, height: {}, mass: {}", self.name, self.height, self.mass)
    }
}