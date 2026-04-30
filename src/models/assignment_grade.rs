use std::fmt;
use chrono::{DateTime, Utc};
use crate::models::assignments::Assignment;
use crate::models::enrollment::Enrollment;

pub struct AssignmentGrade {
    pub enrollment: Enrollment,
    pub assignment: Assignment,
    pub grade: f64,
    pub graded_at: Option<DateTime<Utc>>,
}

impl fmt::Display for AssignmentGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let student_name = self.enrollment.student.full_name().to_string();
        let assignment_name = self.assignment.assignment_name.to_string();
        
        write!(f, "{}'s assignment {} was graded at {}%", student_name, assignment_name, self.grade)
    }
}