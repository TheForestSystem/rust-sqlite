mod models;
mod persistence;
pub mod views;

use crate::models::course::Course;
use crate::models::staff::Staff;
use crate::models::student::Student;
use persistence::context::DbContext;
use persistence::db;
use crate::models::enrollment::Enrollment;

fn main() -> Result<(), rusqlite::Error> {
    let conn = db::open("school.db")?;
    let mut ctx = DbContext::new(&conn);

    // everything is accessible through ctx
    let staff_id = ctx
        .staff
        .insert(&Staff::new("Jane".to_string(), "Smith".to_string()))?;
    let course_id = ctx
        .courses
        .insert(&Course::new(staff_id, "Algebra 101".to_string()))?;
    let student_id = ctx
        .students
        .insert(&Student::new("Jon".to_string(), "Doe".to_string()))?;

    let enrollment = ctx
        .enrollments
        .insert(&Enrollment::new(student_id, course_id))?;
    

    Ok(())
}
