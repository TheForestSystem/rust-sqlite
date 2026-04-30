use chrono::{DateTime, Utc};
use std::fmt;

pub struct Course {
    pub course_id: Option<i64>,
    pub course_name: String,
    pub staff_id: i64,
    pub created_at: Option<DateTime<Utc>>,
}

impl Course {
    pub fn new(staff_id: i64, course_name: String) -> Self {
        Course {
            course_id: None,
            course_name,
            staff_id,
            created_at: None,
        }
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(id) = self.course_id {
            write!(
                f,
                "Course #{}: {} (Staff ID: {})",
                id, self.course_name, self.staff_id
            )
        } else {
            write!(
                f,
                "Course {} (Staff ID: {})",
                self.course_name, self.staff_id
            )
        }
    }
}
