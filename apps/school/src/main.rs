use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut wrapper_service = WrapperService::new(Box::new(PersonManagerImpl { persons: vec![] }));

    let id1 = ok_or_panic(&mut || wrapper_service.add_person("John".to_string(), 25));
    let _ = ok_or_panic(&mut || wrapper_service.add_person("Jane".to_string(), 30));
    let _ = ok_or_panic(&mut || wrapper_service.remove_person(id1));

    wrapper_service.print_all_persons();

    // Counter examples
    let counter1 = Rc::new(Counter::new());
    let counter2 = Rc::clone(&counter1);
    let counter3 = Rc::clone(&counter1);
    let counter4 = counter1.as_ref();

    increase_counter(&counter2);
    increase_counter(&counter2);
    increase_counter(&counter3);
    increase_counter(&counter1);
    increase_counter(&counter2);
    *counter4.count.borrow_mut() += 1;

    println!("Counter1: {}", counter1.count.borrow());
    println!("Counter2: {}", counter2.count.borrow());
    println!("Counter3: {}", counter3.count.borrow());

    // Cow example
    let original = "hello";
    let suffix = " world";

    // In this case, the original string does not end with the suffix, so it's cloned and modified.
    let modified = ensure_suffix(original, suffix);
    println!("Modified: {}", modified);

    // Here, the original string already ends with the suffix, so no cloning is done.
    let already_suffixed = "hello world";
    let unmodified = ensure_suffix(already_suffixed, suffix);
    println!("Unmodified: {}", unmodified);
}

struct Counter {
    count: RefCell<usize>,
}

impl Counter {
    fn new() -> Self {
        Self { count: RefCell::new(0) }
    }
}

fn increase_counter(counter: &Rc<Counter>) {
    *counter.count.borrow_mut() += 1;
}

fn ensure_suffix<'a>(input: &'a str, suffix: &'a str) -> Cow<'a, str> {
    if input.ends_with(suffix) {
        // No need to modify the string, so return a borrowed reference
        Cow::Borrowed(input)
    } else {
        // Need to modify the string, so clone it and append the suffix
        Cow::Owned(format!("{}{}", input, suffix))
    }
}

fn ok_or_panic<TSuccess>(f: &mut dyn FnMut() -> Result<TSuccess, String>) -> TSuccess {
    let res = f();
    if let Err(e) = res {
        println!("Error: {}", e);
        panic!("{}", e);
    }

    res.unwrap()
}

struct WrapperService {
    id_counter: usize,
    person_manager: Box<dyn PersonManager>,
}

impl WrapperService {
    fn new(person_manager: Box<dyn PersonManager>) -> Self {
        Self { person_manager, id_counter: 1 }
    }

    fn add_person(&mut self, name: String, age: u8) -> Result<usize, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if self.person_manager.find_person(self.id_counter).is_ok() {
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

    fn remove_person(&mut self, id: usize) -> Result<(), String> {
        self.person_manager.remove_person(id)
    }

    fn print_all_persons(&self) {
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

#[derive(Debug)]
struct Person {
    id: usize,
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8, id: usize) -> Self {
        Self { name, age, id }
    }
}

trait PersonManager {
    fn find_person(&self, id: usize) -> Result<Option<&Person>, String>;
    fn get_all_persons(&self) -> Result<Vec<&Person>, String>;
    fn add_person(&mut self, person: Person) -> Result<(), String>;
    fn remove_person(&mut self, id: usize) -> Result<(), String>;
}

struct PersonManagerImpl {
    persons: Vec<Person>,
}

impl PersonManager for PersonManagerImpl {
    fn find_person(&self, id: usize) -> Result<Option<&Person>, String> {
        self.persons.iter()
            .find(|person| person.id == id)
            .map(|person| Some(person))
            .ok_or("Person not found".to_string())
    }

    fn get_all_persons(&self) -> Result<Vec<&Person>, String> {
        Ok(self.persons.iter().collect())
    }

    fn add_person(&mut self, person: Person) -> Result<(), String> {
        self.persons.push(person);

        Ok(())
    }

    fn remove_person(&mut self, id: usize) -> Result<(), String> {
        self.persons.retain(|p| p.id != id);

        Ok(())
    }
}