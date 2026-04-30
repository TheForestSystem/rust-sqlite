use chrono::{DateTime, Utc};
use std::fmt;

pub struct Staff {
    pub staff_id: Option<i64>,
    pub staff_first: String,
    pub staff_last: String,
    pub staff_email: String,
    pub staff_title: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Staff {
    pub fn new(first_name: String, last_name: String) -> Self {
        Staff {
            staff_id: None,
            staff_first: first_name.clone(),
            staff_last: last_name.clone(),
            staff_email: format!(
                "{}.{}@foxxything.com",
                first_name.to_lowercase(),
                last_name.to_lowercase()
            ),
            staff_title: "teacher".to_string(),
            created_at: None,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.staff_first, self.staff_last)
    }
}

impl fmt::Display for Staff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(id) = self.staff_id {
            write!(f, "{}: {} {}", id, self.staff_first, self.staff_last)
        } else {
            write!(f, "{} {}", self.staff_first, self.staff_last)
        }
    }
}
