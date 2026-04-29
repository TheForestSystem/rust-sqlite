use rusqlite::{Connection, Error, Statement, named_params};
use crate::persistence::person::Person;

pub struct MyDbContext<'a> {
    pub conn: &'a Connection,
    pub create_person_statement: Option<Statement<'a>>,
    pub fetch_person_statement: Option<Statement<'a>>,
}

impl<'a> MyDbContext<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        MyDbContext {
            conn,
            create_person_statement: None,
            fetch_person_statement: None,
        }
    }

    pub fn create_person(&mut self, name: &str, email: &str) -> Result<i64, Error> {
        if self.create_person_statement.is_none() {
            let stmt = self.conn.prepare(
                "INSERT INTO person (name, email) VALUES (:name, :email)"
            )?;
            self.create_person_statement = Some(stmt);
        }

        self.create_person_statement
            .as_mut()
            .unwrap()
            .execute(named_params! {
                ":name": name,
                ":email": email,
            })?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_persons(&self) -> Result<Vec<Person>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email FROM person ORDER BY id DESC",
        )?;

        let mut rows = stmt.query([])?;
        let mut persons = Vec::new();

        while let Some(row) = rows.next()? {
            persons.push(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
            });
        }

        Ok(persons)
    }

    pub fn fetch_person_by_id(&mut self, id: i32) -> Result<Person, Error> {
        if self.fetch_person_statement.is_none() {
            let stmt = self.conn.prepare(
                "SELECT id, name, email FROM person WHERE id = :id"
            )?;
            self.fetch_person_statement = Some(stmt);
        }

        self.fetch_person_statement
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    // a helper that sets up a fresh in-memory db for each test
    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("
            CREATE TABLE person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                email TEXT NOT NULL
            );
        ").unwrap();
        conn
    }

    #[test]
    fn test_create_and_fetch_person() {
        let conn = setup();
        let mut ctx = MyDbContext::new(&conn);

        let id = ctx.create_person("Alice", "alice@example.com").unwrap();
        let person = ctx.fetch_person_by_id(id as i32).unwrap();

        assert_eq!(person.name, "Alice");
        assert_eq!(person.email, "alice@example.com");
        assert_eq!(person.id, Some(id));
    }

    #[test]
    fn test_fetch_person_not_found() {
        let conn = setup();
        let mut ctx = MyDbContext::new(&conn);

        let result = ctx.fetch_person_by_id(999);
        assert!(result.is_err()); // should return an error for a missing id
    }
}