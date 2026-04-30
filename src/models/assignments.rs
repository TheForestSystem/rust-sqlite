use crate::models::course::Course;
use chrono::{DateTime, Utc};
use std::fmt;

pub struct Assignment {
    pub assignment_id: Option<i64>,
    pub course: Course,
    pub assignment_name: String,
    pub weight: f32,
    pub created_at: Option<DateTime<Utc>>,
}

impl Assignment {
    pub fn new(course: Course, assignment_name: String, weight: Option<f32>) -> Assignment {
        if let Some(weight) = weight {
            Assignment {
                assignment_id: None,
                course,
                assignment_name,
                weight,
                created_at: None,
            }
        } else {
            Assignment {
                assignment_id: None,
                course,
                assignment_name,
                weight: 1.0,
                created_at: None,
            }
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} weighted at {}", self.assignment_name, self.weight)
    }
}
