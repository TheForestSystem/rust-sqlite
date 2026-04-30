use crate::models::assignment::Assignment;
use rusqlite::{Connection, Error, Statement, named_params};

pub struct AssignmentRepo<'a> {
    conn: &'a Connection,
    insert_stmt: Option<Statement<'a>>,
    fetch_by_id_stmt: Option<Statement<'a>>,
    fetch_by_course_stmt: Option<Statement<'a>>,
}

impl<'a> AssignmentRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        AssignmentRepo {
            conn,
            insert_stmt: None,
            fetch_by_id_stmt: None,
            fetch_by_course_stmt: None,
        }
    }

    pub fn insert(&mut self, assignment: &Assignment) -> Result<i64, Error> {
        if self.insert_stmt.is_none() {
            self.insert_stmt = Some(self.conn.prepare(
                "
                INSERT INTO assignments (course_id, assignment_name, weight)
                VALUES (:course_id, :name, :weight)
            ",
            )?);
        }
        self.insert_stmt.as_mut().unwrap().execute(named_params! {
            ":course_id": assignment.course_id,
            ":name":      assignment.assignment_name,
            ":weight":    assignment.weight,
        })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn fetch_by_id(&mut self, id: i64) -> Result<Assignment, Error> {
        if self.fetch_by_id_stmt.is_none() {
            self.fetch_by_id_stmt = Some(self.conn.prepare(
                "
                SELECT assignment_id, course_id, assignment_name, weight, created_at
                FROM assignments WHERE assignment_id = :id
            ",
            )?);
        }
        self.fetch_by_id_stmt
            .as_mut()
            .unwrap()
            .query_row(named_params! { ":id": id }, |row| {
                Ok(Assignment {
                    assignment_id: row.get(0)?,
                    course_id: row.get(1)?,
                    assignment_name: row.get(2)?,
                    weight: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
    }

    pub fn fetch_by_course(&mut self, course_id: i64) -> Result<Vec<Assignment>, Error> {
        if self.fetch_by_course_stmt.is_none() {
            self.fetch_by_course_stmt = Some(self.conn.prepare(
                "
                SELECT assignment_id, course_id, assignment_name, weight, created_at
                FROM assignments WHERE course_id = :course_id ORDER BY assignment_name
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
            result.push(Assignment {
                assignment_id: row.get(0)?,
                course_id: row.get(1)?,
                assignment_name: row.get(2)?,
                weight: row.get(3)?,
                created_at: row.get(4)?,
            });
        }
        Ok(result)
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Error> {
        self.conn.execute(
            "DELETE FROM assignments WHERE assignment_id = :id",
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
    use crate::persistence::course_repo::CourseRepo;
    use crate::persistence::db;
    use crate::persistence::staff_repo::StaffRepo;

    fn setup_with_course(conn: &rusqlite::Connection) -> i64 {
        let mut staff_repo = StaffRepo::new(conn);
        let staff_id = staff_repo
            .insert(&Staff::new("Jane".to_string(), "Smith".to_string()))
            .unwrap();
        let mut course_repo = CourseRepo::new(conn);
        course_repo
            .insert(&Course::new(staff_id, "Algebra 101".to_string()))
            .unwrap()
    }

    #[test]
    fn test_insert_and_fetch() {
        let conn = db::open_in_memory().unwrap();
        let course_id = setup_with_course(&conn);
        let mut repo = AssignmentRepo::new(&conn);
        let assignment = Assignment::new(course_id, "Midterm".to_string(), Some(0.4));
        let id = repo.insert(&assignment).unwrap();
        let fetched = repo.fetch_by_id(id).unwrap();
        assert_eq!(fetched.assignment_name, "Midterm");
        assert_eq!(fetched.weight, 0.4);
    }

    #[test]
    fn test_fetch_by_course() {
        let conn = db::open_in_memory().unwrap();
        let course_id = setup_with_course(&conn);
        let mut repo = AssignmentRepo::new(&conn);
        repo.insert(&Assignment::new(
            course_id,
            "Midterm".to_string(),
            Some(0.4),
        ))
        .unwrap();
        repo.insert(&Assignment::new(course_id, "Final".to_string(), Some(0.6)))
            .unwrap();
        assert_eq!(repo.fetch_by_course(course_id).unwrap().len(), 2);
    }
}
