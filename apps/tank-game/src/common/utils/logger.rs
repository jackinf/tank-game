pub struct Logger;

impl Logger {
    pub fn log(message: &str) {
        println!("{}", message);
    }
}