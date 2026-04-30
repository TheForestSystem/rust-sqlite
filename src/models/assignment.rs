use chrono::{DateTime, Utc};
use std::fmt;

pub struct Assignment {
    pub assignment_id: Option<i64>,
    pub course_id: i64,
    pub assignment_name: String,
    pub weight: f64,
    pub created_at: Option<DateTime<Utc>>,
}

impl Assignment {
    pub fn new(course_id: i64, assignment_name: String, weight: Option<f64>) -> Assignment {
        Assignment {
            assignment_id: None,
            course_id,
            assignment_name,
            weight: weight.unwrap_or(1.0),
            created_at: None,
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} weighted at {}", self.assignment_name, self.weight)
    }
}
