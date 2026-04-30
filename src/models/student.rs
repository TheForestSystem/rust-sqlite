use chrono::{DateTime, Utc};
use std::fmt;

pub struct Student {
    pub student_id: Option<i64>,
    pub student_first: String,
    pub student_last: String,
    pub student_email: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Student {
    pub fn new(first_name: String, last_name: String) -> Self {
        let email = format!(
            "{}.{}@foxxything.com",
            first_name.chars().next().unwrap_or('x').to_lowercase(),
            last_name.to_lowercase()
        );

        Student {
            student_id: None,
            student_first: first_name,
            student_last: last_name,
            student_email: email,
            created_at: None,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.student_first, self.student_last)
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(id) = self.student_id {
            write!(f, "{}: {} {}", id, self.student_first, self.student_last)
        } else {
            write!(f, "{} {}", self.student_first, self.student_last)
        }
    }
}
