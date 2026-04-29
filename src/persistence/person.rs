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

#[cfg(test)]
mod tests {
    use super::*; // brings everything from the parent module into scope

    #[test]
    fn test_person_display_with_id() {
        let person = Person {
            id: Some(1),
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        };
        assert_eq!(format!("{}", person), "1: Alice - alice@example.com");
    }

    #[test]
    fn test_person_display_without_id() {
        let person = Person {
            id: None,
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
        };
        assert_eq!(format!("{}", person), "Alice - alice@example.com");
    }
}