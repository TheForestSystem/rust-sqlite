use crate::models::course::Course;
use rusqlite::{Connection, Error, Statement, named_params};

pub struct CourseRepo<'a> {
    conn: &'a Connection,
    insert_stmt: Option<Statement<'a>>,
    fetch_by_id_stmt: Option<Statement<'a>>,
    fetch_all_stmt: Option<Statement<'a>>,
    fetch_by_staff_stmt: Option<Statement<'a>>,
}

impl<'a> CourseRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        CourseRepo {
            conn,
            insert_stmt: None,
            fetch_by_id_stmt: None,
            fetch_all_stmt: None,
            fetch_by_staff_stmt: None,
        }
    }

    pub fn insert(&mut self, course: &Course) -> Result<i64, Error> {
        if self.insert_stmt.is_none() {
            self.insert_stmt = Some(self.conn.prepare(
                "
                INSERT INTO courses (course_name, staff_id)
                VALUES (:name, :staff_id)
            ",
            )?);
        }
        self.insert_stmt.as_mut().unwrap().execute(named_params! {
            ":name":     course.course_name,
            ":staff_id": course.staff_id,
        })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_id(&mut self, id: i64) -> Result<Course, Error> {
        if self.fetch_by_id_stmt.is_none() {
            self.fetch_by_id_stmt = Some(self.conn.prepare(
                "
                SELECT course_id, course_name, staff_id, created_at
                FROM courses WHERE course_id = :id
            ",
            )?);
        }
        self.fetch_by_id_stmt
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Course {
                    course_id: row.get(0)?,
                    course_name: row.get(1)?,
                    staff_id: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
    }

    pub fn fetch_all(&mut self) -> Result<Vec<Course>, Error> {
        if self.fetch_all_stmt.is_none() {
            self.fetch_all_stmt = Some(self.conn.prepare(
                "
                SELECT course_id, course_name, staff_id, created_at
                FROM courses ORDER BY course_name
            ",
            )?);
        }
        let mut rows = self.fetch_all_stmt.as_mut().unwrap().query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Course {
                course_id: row.get(0)?,
                course_name: row.get(1)?,
                staff_id: row.get(2)?,
                created_at: row.get(3)?,
            });
        }
        Ok(result)
    }

    pub fn fetch_by_staff(&mut self, staff_id: i64) -> Result<Vec<Course>, Error> {
        if self.fetch_by_staff_stmt.is_none() {
            self.fetch_by_staff_stmt = Some(self.conn.prepare(
                "
                SELECT course_id, course_name, staff_id, created_at
                FROM courses WHERE staff_id = :staff_id ORDER BY course_name
            ",
            )?);
        }
        let mut rows = self
            .fetch_by_staff_stmt
            .as_mut()
            .unwrap()
            .query(named_params! { ":staff_id": staff_id })?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(Course {
                course_id: row.get(0)?,
                course_name: row.get(1)?,
                staff_id: row.get(2)?,
                created_at: row.get(3)?,
            });
        }
        Ok(result)
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM courses WHERE course_id = :id",
            named_params! { ":id": id },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::staff::Staff;
    use crate::persistence::db;
    use crate::persistence::staff_repo::StaffRepo;

    fn setup_with_staff(conn: &rusqlite::Connection) -> i64 {
        let mut staff_repo = StaffRepo::new(conn);
        staff_repo
            .insert(&Staff::new("Jane".to_string(), "Smith".to_string()))
            .unwrap()
    }

    #[test]
    fn test_insert_and_fetch() {
        let conn = db::open_in_memory().unwrap();
        let staff_id = setup_with_staff(&conn);
        let mut repo = CourseRepo::new(&conn);
        let course = Course::new(staff_id, "Algebra 101".to_string());
        let id = repo.insert(&course).unwrap();
        let fetched = repo.fetch_by_id(id).unwrap();
        assert_eq!(fetched.course_name, "Algebra 101");
        assert_eq!(fetched.staff_id, staff_id);
    }

    #[test]
    fn test_fetch_by_staff() {
        let conn = db::open_in_memory().unwrap();
        let staff_id = setup_with_staff(&conn);
        let mut repo = CourseRepo::new(&conn);
        repo.insert(&Course::new(staff_id, "Algebra 101".to_string()))
            .unwrap();
        repo.insert(&Course::new(staff_id, "Geometry".to_string()))
            .unwrap();
        assert_eq!(repo.fetch_by_staff(staff_id).unwrap().len(), 2);
    }
}
