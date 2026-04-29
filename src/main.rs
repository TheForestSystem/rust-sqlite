use crate::persistence::my_db_context::MyDbContext;
use crate::persistence::person::Person;
use rusqlite::{Connection, Error};

pub mod persistence;

fn main() -> Result<(), Error> {
    let persons_to_insert: Vec<Person> = vec![
        Person {
            id: None,
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        },
        Person {
            id: None,
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
        },
    ];

    let conn: Connection = Connection::open("myfile.db")?;
    let mut context: MyDbContext = MyDbContext::new(&conn);
    //
    // context.conn.execute_batch("BEGIN TRANSACTION;")?;
    // for p in &persons_to_insert {
    //     context.create_person(&p.name, &p.email)?;
    // }
    // context.conn.execute_batch("COMMIT TRANSACTION;")?;

    let persons: Vec<Person> = context.fetch_persons()?;

    for person in persons {
        println!("{}", person);
    }

    println!("----------");

    let person = context.fetch_person_by_id(1);
    match person {
        Ok(person) => {
            println!("{}", person);
        }
        
        Err(e) => {
            println!("{}", e);
        }
    }

    Ok(())
}