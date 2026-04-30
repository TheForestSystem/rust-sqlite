use crate::persistence::my_db_context::MyDbContext;
use rusqlite::{Connection, Error};

pub mod persistence;
pub mod models;

fn main() -> Result<(), Error> {


    let conn: Connection = Connection::open("myfile.db")?;
    let mut context: MyDbContext = MyDbContext::new(&conn);

    println!("WIP");

    Ok(())
}