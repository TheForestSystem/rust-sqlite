use crate::models::assignment_grade::AssignmentGrade;
use rusqlite::{Connection, Error, Statement, named_params};

pub struct AssignmentGradeRepo<'a> {
    conn: &'a Connection,
    insert_stmt: Option<Statement<'a>>,
    fetch_by_enrollment_stmt: Option<Statement<'a>>,
    update_stmt: Option<Statement<'a>>,
}

impl<'a> AssignmentGradeRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        AssignmentGradeRepo {
            conn,
            insert_stmt: None,
            fetch_by_enrollment_stmt: None,
            update_stmt: None,
        }
    }

    pub fn insert(&mut self, grade: &AssignmentGrade) -> Result<(), Error> {
        if self.insert_stmt.is_none() {
            self.insert_stmt = Some(self.conn.prepare(
                "
                INSERT INTO assignment_grades (enrollment_id, assignment_id, grade)
                VALUES (:enrollment_id, :assignment_id, :grade)
            ",
            )?);
        }
        self.insert_stmt.as_mut().unwrap().execute(named_params! {
            ":enrollment_id": grade.enrollment_id,
            ":assignment_id": grade.assignment_id,
            ":grade":         grade.grade,
        })?;
        Ok(())
    }

    pub fn fetch_by_enrollment(
        &mut self,
        enrollment_id: i64,
    ) -> Result<Vec<AssignmentGrade>, Error> {
        if self.fetch_by_enrollment_stmt.is_none() {
            self.fetch_by_enrollment_stmt = Some(self.conn.prepare(
                "
                SELECT enrollment_id, assignment_id, grade, graded_at
                FROM assignment_grades WHERE enrollment_id = :enrollment_id
            ",
            )?);
        }
        let mut rows = self
            .fetch_by_enrollment_stmt
            .as_mut()
            .unwrap()
            .query(named_params! { ":enrollment_id": enrollment_id })?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(AssignmentGrade {
                enrollment_id: row.get(0)?,
                assignment_id: row.get(1)?,
                grade: row.get(2)?,
                graded_at: row.get(3)?,
            });
        }
        Ok(result)
    }

    pub fn update_grade(
        &mut self,
        enrollment_id: i64,
        assignment_id: i64,
        grade: f64,
    ) -> Result<(), Error> {
        if self.update_stmt.is_none() {
            self.update_stmt = Some(self.conn.prepare(
                "
                UPDATE assignment_grades SET grade = :grade
                WHERE enrollment_id = :enrollment_id AND assignment_id = :assignment_id
            ",
            )?);
        }
        self.update_stmt.as_mut().unwrap().execute(named_params! {
            ":grade":         grade,
            ":enrollment_id": enrollment_id,
            ":assignment_id": assignment_id,
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::assignment::Assignment;
    use crate::models::course::Course;
    use crate::models::enrollment::Enrollment;
    use crate::models::staff::Staff;
    use crate::models::student::Student;
    use crate::persistence::assignment_repo::AssignmentRepo;
    use crate::persistence::course_repo::CourseRepo;
    use crate::persistence::db;
    use crate::persistence::enrollment_repo::EnrollmentRepo;
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
        let mut enrollment_repo = EnrollmentRepo::new(conn);
        let enrollment_id = enrollment_repo
            .insert(&Enrollment {
                enrollment_id: None,
                student_id,
                course_id,
                enrolled_at: None,
            })
            .unwrap();
        let mut assignment_repo = AssignmentRepo::new(conn);
        let assignment_id = assignment_repo
            .insert(&Assignment::new(
                course_id,
                "Midterm".to_string(),
                Some(0.4),
            ))
            .unwrap();
        (enrollment_id, assignment_id)
    }

    #[test]
    fn test_insert_and_fetch() {
        let conn = db::open_in_memory().unwrap();
        let (enrollment_id, assignment_id) = setup(&conn);
        let mut repo = AssignmentGradeRepo::new(&conn);
        repo.insert(&AssignmentGrade {
            enrollment_id,
            assignment_id,
            grade: 85.0,
            graded_at: None,
        })
        .unwrap();
        let grades = repo.fetch_by_enrollment(enrollment_id).unwrap();
        assert_eq!(grades.len(), 1);
        assert_eq!(grades[0].grade, 85.0);
    }

    #[test]
    fn test_update_grade() {
        let conn = db::open_in_memory().unwrap();
        let (enrollment_id, assignment_id) = setup(&conn);
        let mut repo = AssignmentGradeRepo::new(&conn);
        repo.insert(&AssignmentGrade {
            enrollment_id,
            assignment_id,
            grade: 85.0,
            graded_at: None,
        })
        .unwrap();
        repo.update_grade(enrollment_id, assignment_id, 92.0)
            .unwrap();
        let grades = repo.fetch_by_enrollment(enrollment_id).unwrap();
        assert_eq!(grades[0].grade, 92.0);
    }
}
