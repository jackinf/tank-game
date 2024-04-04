use crate::models::person::Person;
use crate::services::person_service::PersonService;
use async_trait::async_trait;
use rusqlite::Connection;

pub struct PersonServiceSqlite {
    pub connection: Connection,
}

impl PersonServiceSqlite {
    pub fn new() -> Self {
        Self {
            connection: Connection::open_in_memory().unwrap(),
        }
    }
}

#[async_trait]
impl PersonService for PersonServiceSqlite {
    fn find_person(&self, id: usize) -> Result<Option<Person>, String> {
        let stmt_result = self
            .connection
            .prepare("SELECT id, name, age FROM person WHERE id = ?1");
        if let Err(e) = stmt_result {
            return Err(e.to_string());
        }
        let mut stmt = stmt_result.unwrap();

        let person_iter_result = stmt.query_map([id], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        });
        if let Err(e) = person_iter_result {
            return Err(e.to_string());
        }
        let person_iter = person_iter_result.unwrap();

        let person = person_iter.into_iter().next();
        match person {
            Some(Ok(person)) => Ok(Some(person.clone())),
            Some(Err(e)) => Err(e.to_string()),
            None => Ok(None),
        }
    }

    fn get_all_persons(&self) -> Result<Vec<Person>, String> {
        let stmt_result = self.connection.prepare("SELECT id, name, age FROM person");
        if let Err(e) = stmt_result {
            return Err(e.to_string());
        }
        let mut stmt = stmt_result.unwrap();

        let person_iter_result = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        });
        if let Err(e) = person_iter_result {
            return Err(e.to_string());
        }
        let person_iter = person_iter_result.unwrap();

        let mut persons = Vec::new();
        for person in person_iter {
            persons.push(person.unwrap());
        }

        Ok(persons)
    }

    fn add_person(&mut self, person: Person) -> Result<usize, String> {
        let result = self.connection.execute(
            "INSERT INTO person (name, age) VALUES (?1, ?2)",
            (&person.name, &person.age),
        );
        if let Err(e) = result {
            return Err(e.to_string());
        }
        let new_id = result.unwrap();

        Ok(new_id)
    }

    fn remove_person(&mut self, id: usize) -> Result<(), String> {
        let result = self
            .connection
            .execute("DELETE FROM person WHERE id = ?1", [id]);
        if let Err(e) = result {
            return Err(e.to_string());
        }

        Ok(())
    }
}
