use std::fmt;
use chrono::{DateTime, Utc};

pub struct AssignmentGrade {
    pub enrollment_id: i64,    // just the foreign key
    pub assignment_id: i64,    // just the foreign key
    pub grade: f64,
    pub graded_at: Option<DateTime<Utc>>,
}

impl fmt::Display for AssignmentGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enrollment {} - Assignment {} graded at {}%",
               self.enrollment_id, self.assignment_id, self.grade)
    }
}