use crate::models::enrollment::Enrollment;
use rusqlite::{Connection, Error, Statement, named_params};

pub struct EnrollmentRepo<'a> {
    conn: &'a Connection,
    insert_stmt: Option<Statement<'a>>,
    fetch_by_id_stmt: Option<Statement<'a>>,
    fetch_by_student_stmt: Option<Statement<'a>>,
    fetch_by_course_stmt: Option<Statement<'a>>,
}

impl<'a> EnrollmentRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        EnrollmentRepo {
            conn,
            insert_stmt: None,
            fetch_by_id_stmt: None,
            fetch_by_student_stmt: None,
            fetch_by_course_stmt: None,
        }
    }

    pub fn insert(&mut self, enrollment: &Enrollment) -> Result<i64, Error> {
        if self.insert_stmt.is_none() {
            self.insert_stmt = Some(self.conn.prepare(
                "
                INSERT INTO enrollments (student_id, course_id)
                VALUES (:student_id, :course_id)
            ",
            )?);
        }
        self.insert_stmt.as_mut().unwrap().execute(named_params! {
            ":student_id": enrollment.student_id,
            ":course_id":  enrollment.course_id,
        })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_id(&mut self, id: i64) -> Result<Enrollment, Error> {
        if self.fetch_by_id_stmt.is_none() {
            self.fetch_by_id_stmt = Some(self.conn.prepare(
                "
                SELECT enrollment_id, student_id, course_id, enrolled_at
                FROM enrollments WHERE enrollment_id = :id
            ",
            )?);
        }
        self.fetch_by_id_stmt
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Enrollment {
                    enrollment_id: row.get(0)?,
                    student_id: row.get(1)?,
                    course_id: row.get(2)?,
                    enrolled_at: row.get(3)?,
                })
            })
    }

    pub fn fetch_by_student(&mut self, student_id: i64) -> Result<Vec<Enrollment>, Error> {
        if self.fetch_by_student_stmt.is_none() {
            self.fetch_by_student_stmt = Some(self.conn.prepare(
                "
                SELECT enrollment_id, student_id, course_id, enrolled_at
                FROM enrollments WHERE student_id = :student_id
            ",
            )?);
        }
        let mut rows = self
            .fetch_by_student_stmt
            .as_mut()
            .unwrap()
            .query(named_params! { ":student_id": student_id })?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Enrollment {
                enrollment_id: row.get(0)?,
                student_id: row.get(1)?,
                course_id: row.get(2)?,
                enrolled_at: row.get(3)?,
            });
        }
        Ok(result)
    }

    pub fn fetch_by_course(&mut self, course_id: i64) -> Result<Vec<Enrollment>, Error> {
        if self.fetch_by_course_stmt.is_none() {
            self.fetch_by_course_stmt = Some(self.conn.prepare(
                "
                SELECT enrollment_id, student_id, course_id, enrolled_at
                FROM enrollments WHERE course_id = :course_id
            ",
            )?);
        }
        let mut rows = self
            .fetch_by_course_stmt
            .as_mut()
            .unwrap()
            .query(named_params! { ":course_id": course_id })?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Enrollment {
                enrollment_id: row.get(0)?,
                student_id: row.get(1)?,
                course_id: row.get(2)?,
                enrolled_at: row.get(3)?,
            });
        }
        Ok(result)
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM enrollments WHERE enrollment_id = :id",
            named_params! { ":id": id },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::course::Course;
    use crate::models::staff::Staff;
    use crate::models::student::Student;
    use crate::persistence::course_repo::CourseRepo;
    use crate::persistence::db;
    use crate::persistence::staff_repo::StaffRepo;
    use crate::persistence::student_repo::StudentRepo;

    fn setup(conn: &rusqlite::Connection) -> (i64, i64) {
        let mut staff_repo = StaffRepo::new(conn);
        let staff_id = staff_repo
            .insert(&Staff::new("Jane".to_string(), "Smith".to_string()))
            .unwrap();
        let mut course_repo = CourseRepo::new(conn);
        let course_id = course_repo
            .insert(&Course::new(staff_id, "Algebra 101".to_string()))
            .unwrap();
        let mut student_repo = StudentRepo::new(conn);
        let student_id = student_repo
            .insert(&Student::new("Jon".to_string(), "Doe".to_string()))
            .unwrap();
        (course_id, student_id)
    }

    #[test]
    fn test_insert_and_fetch() {
        let conn = db::open_in_memory().unwrap();
        let (course_id, student_id) = setup(&conn);
        let mut repo = EnrollmentRepo::new(&conn);
        let enrollment = Enrollment {
            enrollment_id: None,
            student_id,
            course_id,
            enrolled_at: None,
        };
        let id = repo.insert(&enrollment).unwrap();
        let fetched = repo.fetch_by_id(id).unwrap();
        assert_eq!(fetched.student_id, student_id);
        assert_eq!(fetched.course_id, course_id);
    }

    #[test]
    fn test_duplicate_enrollment_fails() {
        let conn = db::open_in_memory().unwrap();
        let (course_id, student_id) = setup(&conn);
        let mut repo = EnrollmentRepo::new(&conn);
        let enrollment = Enrollment {
            enrollment_id: None,
            student_id,
            course_id,
            enrolled_at: None,
        };
        repo.insert(&enrollment).unwrap();
        // second insert should fail due to unique constraint
        let result = repo.insert(&Enrollment {
            enrollment_id: None,
            student_id,
            course_id,
            enrolled_at: None,
        });
        assert!(result.is_err());
    }
}
