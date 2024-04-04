use crate::models::person::Person;
use async_trait::async_trait;

#[async_trait]
pub trait PersonService {
    fn find_person(&self, id: usize) -> Result<Option<Person>, String>;
    fn get_all_persons(&self) -> Result<Vec<Person>, String>;
    fn add_person(&mut self, person: Person) -> Result<usize, String>;
    fn remove_person(&mut self, id: usize) -> Result<(), String>;
}
