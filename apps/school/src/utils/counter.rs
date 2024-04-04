use std::cell::RefCell;
use std::rc::Rc;

pub struct Counter {
    count: RefCell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            count: RefCell::new(0),
        }
    }

    pub fn increase_counter(counter: &Rc<Counter>) {
        *counter.count.borrow_mut() += 1;
    }

    pub fn get_count(counter: &Rc<Counter>) -> usize {
        *counter.count.borrow()
    }
}
