use crate::models::person::Person;
use crate::services::person_service::PersonService;
use async_trait::async_trait;

pub struct PersonServiceInMemory {
    persons: Vec<Person>,
}

impl PersonServiceInMemory {
    pub fn new() -> Self {
        Self {
            persons: Vec::new(),
        }
    }
}

#[async_trait]
impl PersonService for PersonServiceInMemory {
    fn find_person(&self, id: usize) -> Result<Option<Person>, String> {
        // find a person
        let person = self.persons.iter().find(|person| person.id == id);

        // deref
        match person {
            Some(person) => Ok(Some(person.clone())),
            None => Ok(None),
        }
    }

    fn get_all_persons(&self) -> Result<Vec<Person>, String> {
        Ok(self.persons.iter().map(|person| person.clone()).collect())
    }

    fn add_person(&mut self, person: Person) -> Result<usize, String> {
        let id = person.id.clone();
        self.persons.push(person);

        Ok(id)
    }

    fn remove_person(&mut self, id: usize) -> Result<(), String> {
        self.persons.retain(|p| p.id != id);

        Ok(())
    }
}
