use crate::models::person::Person;
use crate::services::person_service::PersonService;

pub struct WrapperService {
    id_counter: usize,
    person_manager: Box<dyn PersonService>,
}

impl WrapperService {
    pub fn new(person_manager: Box<dyn PersonService>) -> Self {
        Self {
            person_manager,
            id_counter: 1,
        }
    }

    pub fn add_person(&mut self, name: String, age: u8) -> Result<usize, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if let Ok(Some(_)) = self.person_manager.find_person(self.id_counter) {
            return Err("Person with this id already exists".to_string());
        }

        let id = self.id_counter;
        let res = self.person_manager.add_person(Person::new(name, age, id));
        if let Err(e) = res {
            return Err(e);
        }
        self.id_counter += 1;

        Ok(id)
    }

    pub fn remove_person(&mut self, id: usize) -> Result<(), String> {
        self.person_manager.remove_person(id)
    }

    pub fn print_all_persons(&self) {
        let persons = self.person_manager.get_all_persons();
        if let Err(e) = persons {
            println!("Error: {}", e);
            return;
        }

        for person in persons.unwrap() {
            println!("{:?}", person);
        }
    }
}
