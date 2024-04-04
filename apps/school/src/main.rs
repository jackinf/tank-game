mod services {
    pub mod person_service;
    pub mod person_service_in_memory;
    pub mod person_service_sqlite;
    pub mod wrapper_service;
}
mod models {
    pub mod person;
}
mod utils {
    pub mod counter;
    pub mod ensure_suffix;
    pub mod ok_or_panic;
}

use crate::services::person_service::PersonService;
use crate::services::person_service_in_memory::PersonServiceInMemory;
use crate::services::wrapper_service::WrapperService;
use crate::utils::counter::Counter;
use crate::utils::ensure_suffix::ensure_suffix;
use crate::utils::ok_or_panic::ok_or_panic;
use std::rc::Rc;

fn main() {
    let mut wrapper_service = WrapperService::new(Box::new(PersonServiceInMemory::new()));

    let id1 = ok_or_panic(&mut || wrapper_service.add_person("John".to_string(), 25));
    let _ = ok_or_panic(&mut || wrapper_service.add_person("Jane".to_string(), 30));
    let _ = ok_or_panic(&mut || wrapper_service.remove_person(id1));

    wrapper_service.print_all_persons();

    // Counter examples
    let counter1 = Rc::new(Counter::new());
    let counter2 = Rc::clone(&counter1);
    let counter3 = Rc::clone(&counter1);
    // let counter4 = counter1.as_ref();

    Counter::increase_counter(&counter2);
    Counter::increase_counter(&counter2);
    Counter::increase_counter(&counter3);
    Counter::increase_counter(&counter1);
    Counter::increase_counter(&counter2);

    println!("Counter1: {}", Counter::get_count(&counter1));
    println!("Counter2: {}", Counter::get_count(&counter2));
    println!("Counter3: {}", Counter::get_count(&counter3));

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
