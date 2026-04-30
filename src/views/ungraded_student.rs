use std::fmt;

pub struct UngradedStudent {
    pub student_id:      i64,
    pub student_name:    String,
    pub course_name:     String,
    pub assignment_name: String,
}

impl fmt::Display for UngradedStudent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has not been graded for {} in {}",
               self.student_name,
               self.assignment_name,
               self.course_name,
        )
    }
}