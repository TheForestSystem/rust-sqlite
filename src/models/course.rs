use std::fmt;
use chrono::{DateTime, Utc};
use crate::models::staff::Staff;

pub struct Course {
    pub course_id: Option<i64>,
    pub course_name: String,
    pub course_instructor: Staff,
    pub created_at: Option<DateTime<Utc>>
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(id) = self.course_id {
            write!(f, "Course #{}: {} is taught by: {}", id, self.course_name, self.course_instructor.full_name())
        } else {
            write!(f, "Course {} is taught by: {}", self.course_name, self.course_instructor.full_name())
        }
    }
}