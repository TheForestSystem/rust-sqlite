use chrono::{DateTime, Utc};
use std::fmt;

pub struct Enrollment {
    pub enrollment_id: Option<i64>,
    pub student_id: i64, // just the foreign key
    pub course_id: i64,  // just the foreign key
    pub enrolled_at: Option<DateTime<Utc>>,
}

impl Enrollment {
    pub fn new(student_id: i64, course_id: i64) -> Self {
        Enrollment {
            enrollment_id: None,
            student_id,
            course_id,
            enrolled_at: None,
        }
    }
}

impl fmt::Display for Enrollment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Enrollment #{:?} - Student {} in Course {}",
            self.enrollment_id, self.student_id, self.course_id
        )
    }
}
