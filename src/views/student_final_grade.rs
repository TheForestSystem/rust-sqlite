use std::fmt;

pub struct StudentFinalGrade {
    pub student_id:         i64,
    pub student_name:       String,
    pub course_id:          i64,
    pub course_name:        String,
    pub teacher_name:       String,
    pub final_grade:        f64,
    pub assignments_graded: i64,
}

impl fmt::Display for StudentFinalGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {} | {:.1}% ({} assignments)",
               self.student_name,
               self.course_name,
               self.teacher_name,
               self.final_grade,
               self.assignments_graded,
        )
    }
}