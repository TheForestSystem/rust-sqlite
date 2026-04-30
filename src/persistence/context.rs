// persistence/context.rs
use rusqlite::Connection;
use crate::persistence::{
    staff_repo::StaffRepo,
    student_repo::StudentRepo,
    course_repo::CourseRepo,
    assignment_repo::AssignmentRepo,
    enrollment_repo::EnrollmentRepo,
    assignment_grade_repo::AssignmentGradeRepo
};

pub struct DbContext<'a> {
    pub staff:            StaffRepo<'a>,
    pub students:         StudentRepo<'a>,
    pub courses:          CourseRepo<'a>,
    pub assignments:      AssignmentRepo<'a>,
    pub enrollments:      EnrollmentRepo<'a>,
    pub assignment_grades: AssignmentGradeRepo<'a>
}

impl<'a> DbContext<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        DbContext {
            staff:             StaffRepo::new(conn),
            students:          StudentRepo::new(conn),
            courses:           CourseRepo::new(conn),
            assignments:       AssignmentRepo::new(conn),
            enrollments:       EnrollmentRepo::new(conn),
            assignment_grades: AssignmentGradeRepo::new(conn)
        }
    }
}