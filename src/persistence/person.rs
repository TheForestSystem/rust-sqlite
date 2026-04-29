use std::fmt;

pub struct Person {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(id) = self.id {
            write!(f, "{}: {} - {}", id, self.name, self.email)
        } else {
            write!(f, "{} - {}", self.name, self.email)
        }
    }
}