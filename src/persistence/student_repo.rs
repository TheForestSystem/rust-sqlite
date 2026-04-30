use rusqlite::{Connection, Error, Statement, named_params};
use crate::models::student::Student;

pub struct StudentRepo<'a> {
    conn: &'a Connection,
    insert_stmt: Option<Statement<'a>>,
    fetch_by_id_stmt: Option<Statement<'a>>,
    fetch_all_stmt: Option<Statement<'a>>,
}

impl<'a> StudentRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        StudentRepo {
            conn,
            insert_stmt: None,
            fetch_by_id_stmt: None,
            fetch_all_stmt: None,
        }
    }

    // INSERT a new student, returns the new row's ID
    pub fn insert(&mut self, student: &Student) -> Result<i64, Error> {
        if self.insert_stmt.is_none() {
            let stmt = self.conn.prepare("
                INSERT INTO student (student_first, student_last, student_email)
                VALUES (:first, :last, :email)
            ")?;
            self.insert_stmt = Some(stmt);
        }

        self.insert_stmt
            .as_mut()
            .unwrap()
            .execute(named_params! {
                ":first": student.student_first,
                ":last":  student.student_last,
                ":email": student.student_email,
            })?;

        Ok(self.conn.last_insert_rowid())
    }

    // FETCH a single student by ID
    pub fn fetch_by_id(&mut self, id: i64) -> Result<Student, Error> {
        if self.fetch_by_id_stmt.is_none() {
            let stmt = self.conn.prepare("
                SELECT student_id, student_first, student_last, student_email, created_at
                FROM student
                WHERE student_id = :id
            ")?;
            self.fetch_by_id_stmt = Some(stmt);
        }

        self.fetch_by_id_stmt
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Student {
                    student_id:    row.get(0)?,
                    student_first: row.get(1)?,
                    student_last:  row.get(2)?,
                    student_email: row.get(3)?,
                    created_at:    row.get(4)?,
                })
            })
    }

    // FETCH all students
    pub fn fetch_all(&mut self) -> Result<Vec<Student>, Error> {
        if self.fetch_all_stmt.is_none() {
            let stmt = self.conn.prepare("
                SELECT student_id, student_first, student_last, student_email, created_at
                FROM student
                ORDER BY student_last, student_first
            ")?;
            self.fetch_all_stmt = Some(stmt);
        }

        let mut rows = self.fetch_all_stmt
            .as_mut()
            .unwrap()
            .query([])?;

        let mut students = Vec::new();
        while let Some(row) = rows.next()? {
            students.push(Student {
                student_id:    row.get(0)?,
                student_first: row.get(1)?,
                student_last:  row.get(2)?,
                student_email: row.get(3)?,
                created_at:    None,
            });
        }

        Ok(students)
    }

    // DELETE a student by ID
    pub fn delete(&mut self, id: i64) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM student WHERE student_id = :id",
            named_params! { ":id": id },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::db;

    fn setup() -> Connection {
        db::open_in_memory().unwrap()
    }

    #[test]
    fn test_insert_and_fetch() {
        let conn = setup();
        let mut repo = StudentRepo::new(&conn);

        let student = Student::new("Jon".to_string(), "Doe".to_string());
        let id = repo.insert(&student).unwrap();

        let fetched = repo.fetch_by_id(id).unwrap();
        assert_eq!(fetched.student_first, "Jon");
        assert_eq!(fetched.student_last, "Doe");
        assert_eq!(fetched.student_email, "j.doe@foxxything.com");
    }

    #[test]
    fn test_fetch_all() {
        let conn = setup();
        let mut repo = StudentRepo::new(&conn);

        repo.insert(&Student::new("Jon".to_string(), "Doe".to_string())).unwrap();
        repo.insert(&Student::new("Jane".to_string(), "Smith".to_string())).unwrap();

        let all = repo.fetch_all().unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_delete() {
        let conn = setup();
        let mut repo = StudentRepo::new(&conn);

        let id = repo.insert(&Student::new("Jon".to_string(), "Doe".to_string())).unwrap();
        repo.delete(id).unwrap();

        let result = repo.fetch_by_id(id);
        assert!(result.is_err());
    }
}