use std::fmt;

pub struct CourseAssignmentAverage {
    pub course_id:       i64,
    pub course_name:     String,
    pub assignment_id:   i64,
    pub assignment_name: String,
    pub weight:          f64,
    pub class_average:   Option<f64>,
    pub students_graded: i64,
}

impl fmt::Display for CourseAssignmentAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let avg = self.class_average
            .map(|a| format!("{:.1}%", a))
            .unwrap_or("No grades yet".to_string());
        write!(f, "{} | {} | Avg: {} | {} students graded",
               self.course_name,
               self.assignment_name,
               avg,
               self.students_graded,
        )
    }
}