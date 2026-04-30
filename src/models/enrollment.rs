use std::fmt;
use chrono::{DateTime, Utc};
use crate::models::course::Course;
use crate::models::student::Student;

pub struct Enrollment {
    pub enrollment_id: Option<i64>,
    pub student: Student,
    pub course: Course,
    pub created_at: Option<DateTime<Utc>>,
}

impl fmt::Display for Enrollment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is enrolled in {}", self.student.full_name(), self.course.course_name)
    }
}