mod models;
mod persistence;

use rusqlite::Error;
use models::student::Student;
use persistence::db;
use persistence::student_repo::StudentRepo;

fn main() -> Result<(), Error> {
    // open the database - creates the file and all tables if they don't exist
    let conn = db::open("school.db")?;

    // create the repo, passing in a reference to the connection
    let mut student_repo = StudentRepo::new(&conn);

    // create a new student using the constructor
    let jon = Student::new("Jon".to_string(), "Doe".to_string());
    println!("Before insert: {}", jon); // no id yet

    // insert into the database, get back the new id
    let id = student_repo.insert(&jon)?;
    println!("Inserted with id: {}", id);

    // fetch back by id
    let fetched = student_repo.fetch_by_id(id)?;
    println!("Fetched: {}", fetched); // now has an id

    // fetch all students
    let all = student_repo.fetch_all()?;
    println!("Total students: {}", all.len());
    for s in &all {
        println!("  - {}", s);
    }

    // delete
    student_repo.delete(id)?;
    println!("Deleted student {}", id);

    Ok(())
}