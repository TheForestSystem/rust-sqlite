use crate::models::student::Student;
use rusqlite::{Connection, Error, Statement, named_params};

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

    pub fn insert(&mut self, student: &Student) -> Result<i64, Error> {
        if self.insert_stmt.is_none() {
            self.insert_stmt = Some(self.conn.prepare(
                "
                INSERT INTO students (student_first, student_last, student_email)
                VALUES (:first, :last, :email)
            ",
            )?);
        }
        self.insert_stmt.as_mut().unwrap().execute(named_params! {
            ":first": student.student_first,
            ":last":  student.student_last,
            ":email": student.student_email,
        })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_id(&mut self, id: i64) -> Result<Student, Error> {
        if self.fetch_by_id_stmt.is_none() {
            self.fetch_by_id_stmt = Some(self.conn.prepare(
                "
                SELECT student_id, student_first, student_last, student_email, created_at
                FROM students WHERE student_id = :id
            ",
            )?);
        }
        self.fetch_by_id_stmt
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Student {
                    student_id: row.get(0)?,
                    student_first: row.get(1)?,
                    student_last: row.get(2)?,
                    student_email: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
    }

    pub fn fetch_all(&mut self) -> Result<Vec<Student>, Error> {
        if self.fetch_all_stmt.is_none() {
            self.fetch_all_stmt = Some(self.conn.prepare(
                "
                SELECT student_id, student_first, student_last, student_email, created_at
                FROM students ORDER BY student_last, student_first
            ",
            )?);
        }
        let mut rows = self.fetch_all_stmt.as_mut().unwrap().query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Student {
                student_id: row.get(0)?,
                student_first: row.get(1)?,
                student_last: row.get(2)?,
                student_email: row.get(3)?,
                created_at: row.get(4)?,
            });
        }
        Ok(result)
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM students WHERE student_id = :id",
            named_params! { ":id": id },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::db;

    #[test]
    fn test_insert_and_fetch() {
        let conn = db::open_in_memory().unwrap();
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
        let conn = db::open_in_memory().unwrap();
        let mut repo = StudentRepo::new(&conn);
        repo.insert(&Student::new("Jon".to_string(), "Doe".to_string()))
            .unwrap();
        repo.insert(&Student::new("Jane".to_string(), "Smith".to_string()))
            .unwrap();
        assert_eq!(repo.fetch_all().unwrap().len(), 2);
    }

    #[test]
    fn test_delete() {
        let conn = db::open_in_memory().unwrap();
        let mut repo = StudentRepo::new(&conn);
        let id = repo
            .insert(&Student::new("Jon".to_string(), "Doe".to_string()))
            .unwrap();
        repo.delete(id).unwrap();
        assert!(repo.fetch_by_id(id).is_err());
    }
}
