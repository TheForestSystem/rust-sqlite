use rusqlite::{Connection, Error, Statement};
use crate::views::student_final_grade::StudentFinalGrade;
use crate::views::course_assignment_average::CourseAssignmentAverage;
use crate::views::ungraded_student::UngradedStudent;

pub struct ViewRepo<'a> {
    conn:                     &'a Connection,
    final_grades_stmt:        Option<Statement<'a>>,
    assignment_averages_stmt: Option<Statement<'a>>,
    ungraded_stmt:            Option<Statement<'a>>,
}

impl<'a> ViewRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        ViewRepo {
            conn,
            final_grades_stmt:        None,
            assignment_averages_stmt: None,
            ungraded_stmt:            None,
        }
    }

    pub fn fetch_final_grades(&mut self) -> Result<Vec<StudentFinalGrade>, Error> {
        if self.final_grades_stmt.is_none() {
            self.final_grades_stmt = Some(self.conn.prepare("
                SELECT student_id, student_name, course_id, course_name,
                       teacher_name, final_grade, assignments_graded
                FROM student_final_grades
                ORDER BY student_name, course_name
            ")?);
        }
        let mut rows = self.final_grades_stmt.as_mut().unwrap().query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(StudentFinalGrade {
                student_id:         row.get(0)?,
                student_name:       row.get(1)?,
                course_id:          row.get(2)?,
                course_name:        row.get(3)?,
                teacher_name:       row.get(4)?,
                final_grade:        row.get(5)?,
                assignments_graded: row.get(6)?,
            });
        }
        Ok(result)
    }

    pub fn fetch_final_grades_for_student(&mut self, student_id: i64) -> Result<Vec<StudentFinalGrade>, Error> {
        let mut stmt = self.conn.prepare("
            SELECT student_id, student_name, course_id, course_name,
                   teacher_name, final_grade, assignments_graded
            FROM student_final_grades
            WHERE student_id = ?1
        ")?;
        let mut rows = stmt.query([student_id])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(StudentFinalGrade {
                student_id:         row.get(0)?,
                student_name:       row.get(1)?,
                course_id:          row.get(2)?,
                course_name:        row.get(3)?,
                teacher_name:       row.get(4)?,
                final_grade:        row.get(5)?,
                assignments_graded: row.get(6)?,
            });
        }
        Ok(result)
    }

    pub fn fetch_assignment_averages(&mut self) -> Result<Vec<CourseAssignmentAverage>, Error> {
        if self.assignment_averages_stmt.is_none() {
            self.assignment_averages_stmt = Some(self.conn.prepare("
                SELECT course_id, course_name, assignment_id, assignment_name,
                       weight, class_average, students_graded
                FROM course_assignment_averages
                ORDER BY course_name, assignment_name
            ")?);
        }
        let mut rows = self.assignment_averages_stmt.as_mut().unwrap().query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(CourseAssignmentAverage {
                course_id:       row.get(0)?,
                course_name:     row.get(1)?,
                assignment_id:   row.get(2)?,
                assignment_name: row.get(3)?,
                weight:          row.get(4)?,
                class_average:   row.get(5)?,
                students_graded: row.get(6)?,
            });
        }
        Ok(result)
    }

    pub fn fetch_ungraded_students(&mut self) -> Result<Vec<UngradedStudent>, Error> {
        if self.ungraded_stmt.is_none() {
            self.ungraded_stmt = Some(self.conn.prepare("
                SELECT student_id, student_name, course_name, assignment_name
                FROM ungraded_students
                ORDER BY student_name
            ")?);
        }
        let mut rows = self.ungraded_stmt.as_mut().unwrap().query([])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            result.push(UngradedStudent {
                student_id:      row.get(0)?,
                student_name:    row.get(1)?,
                course_name:     row.get(2)?,
                assignment_name: row.get(3)?,
            });
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::db;
    use crate::persistence::staff_repo::StaffRepo;
    use crate::persistence::course_repo::CourseRepo;
    use crate::persistence::student_repo::StudentRepo;
    use crate::persistence::enrollment_repo::EnrollmentRepo;
    use crate::persistence::assignment_repo::AssignmentRepo;
    use crate::persistence::assignment_grade_repo::AssignmentGradeRepo;
    use crate::models::staff::Staff;
    use crate::models::course::Course;
    use crate::models::student::Student;
    use crate::models::enrollment::Enrollment;
    use crate::models::assignment::Assignment;
    use crate::models::assignment_grade::AssignmentGrade;

    // Seeds a complete set of data and returns (student_id, course_id, enrollment_id, assignment_id)
    fn seed(conn: &Connection) -> (i64, i64, i64, i64) {
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
            .insert(&Enrollment { enrollment_id: None, student_id, course_id, enrolled_at: None })
            .unwrap();

        let mut assignment_repo = AssignmentRepo::new(conn);
        let assignment_id = assignment_repo
            .insert(&Assignment::new(course_id, "Midterm".to_string(), Some(0.4)))
            .unwrap();

        (student_id, course_id, enrollment_id, assignment_id)
    }

    // Seeds a grade on top of the base seed
    fn seed_with_grade(conn: &Connection) -> (i64, i64, i64, i64) {
        let ids = seed(conn);
        let (_, _, enrollment_id, assignment_id) = ids;
        let mut grade_repo = AssignmentGradeRepo::new(conn);
        grade_repo
            .insert(&AssignmentGrade {
                enrollment_id,
                assignment_id,
                grade: 85.0,
                graded_at: None,
            })
            .unwrap();
        ids
    }

    #[test]
    fn test_final_grades_empty_without_grades() {
        let conn = db::open_in_memory().unwrap();
        seed(&conn); // enrolled but no grades submitted
        let mut repo = ViewRepo::new(&conn);
        let grades = repo.fetch_final_grades().unwrap();
        // view only shows students who have at least one grade
        assert_eq!(grades.len(), 0);
    }

    #[test]
    fn test_final_grades_returns_after_grading() {
        let conn = db::open_in_memory().unwrap();
        seed_with_grade(&conn);
        let mut repo = ViewRepo::new(&conn);
        let grades = repo.fetch_final_grades().unwrap();
        assert_eq!(grades.len(), 1);
        assert_eq!(grades[0].student_name, "Jon Doe");
        assert_eq!(grades[0].course_name, "Algebra 101");
        assert_eq!(grades[0].teacher_name, "Jane Smith");
        assert_eq!(grades[0].final_grade, 85.0);
        assert_eq!(grades[0].assignments_graded, 1);
    }

    #[test]
    fn test_fetch_final_grades_for_student() {
        let conn = db::open_in_memory().unwrap();
        let (student_id, _, _, _) = seed_with_grade(&conn);
        let mut repo = ViewRepo::new(&conn);
        let grades = repo.fetch_final_grades_for_student(student_id).unwrap();
        assert_eq!(grades.len(), 1);
        assert_eq!(grades[0].student_id, student_id);
    }

    #[test]
    fn test_fetch_final_grades_for_wrong_student_returns_empty() {
        let conn = db::open_in_memory().unwrap();
        seed_with_grade(&conn);
        let mut repo = ViewRepo::new(&conn);
        let grades = repo.fetch_final_grades_for_student(999).unwrap();
        assert_eq!(grades.len(), 0);
    }

    #[test]
    fn test_assignment_averages_no_grades() {
        let conn = db::open_in_memory().unwrap();
        seed(&conn); // assignment exists but no grades yet
        let mut repo = ViewRepo::new(&conn);
        let averages = repo.fetch_assignment_averages().unwrap();
        assert_eq!(averages.len(), 1);
        assert_eq!(averages[0].assignment_name, "Midterm");
        assert_eq!(averages[0].students_graded, 0);
        assert!(averages[0].class_average.is_none()); // NULL because no grades yet
    }

    #[test]
    fn test_assignment_averages_with_grade() {
        let conn = db::open_in_memory().unwrap();
        seed_with_grade(&conn);
        let mut repo = ViewRepo::new(&conn);
        let averages = repo.fetch_assignment_averages().unwrap();
        assert_eq!(averages.len(), 1);
        assert_eq!(averages[0].class_average, Some(85.0));
        assert_eq!(averages[0].students_graded, 1);
    }

    #[test]
    fn test_ungraded_students_shows_before_grading() {
        let conn = db::open_in_memory().unwrap();
        seed(&conn); // enrolled, assignment exists, but not graded
        let mut repo = ViewRepo::new(&conn);
        let ungraded = repo.fetch_ungraded_students().unwrap();
        assert_eq!(ungraded.len(), 1);
        assert_eq!(ungraded[0].student_name, "Jon Doe");
        assert_eq!(ungraded[0].assignment_name, "Midterm");
    }

    #[test]
    fn test_ungraded_students_empty_after_grading() {
        let conn = db::open_in_memory().unwrap();
        seed_with_grade(&conn); // fully graded
        let mut repo = ViewRepo::new(&conn);
        let ungraded = repo.fetch_ungraded_students().unwrap();
        assert_eq!(ungraded.len(), 0);
    }

    #[test]
    fn test_weighted_final_grade() {
        let conn = db::open_in_memory().unwrap();
        let (_, course_id, enrollment_id, _) = seed_with_grade(&conn); // midterm 85, weight 0.4

        // add a final exam with weight 0.6
        let mut assignment_repo = AssignmentRepo::new(&conn);
        let final_id = assignment_repo
            .insert(&Assignment::new(course_id, "Final".to_string(), Some(0.6)))
            .unwrap();

        let mut grade_repo = AssignmentGradeRepo::new(&conn);
        grade_repo
            .insert(&AssignmentGrade {
                enrollment_id,
                assignment_id: final_id,
                grade: 95.0,
                graded_at: None,
            })
            .unwrap();

        let mut repo = ViewRepo::new(&conn);
        let grades = repo.fetch_final_grades().unwrap();

        // weighted average: (85 * 0.4 + 95 * 0.6) / (0.4 + 0.6) = 91.0
        assert_eq!(grades[0].final_grade, 91.0);
        assert_eq!(grades[0].assignments_graded, 2);
    }
}