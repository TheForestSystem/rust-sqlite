use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum MainMenuAction {
    Staff,
    Student,
    Courses,
    TeacherLogin,
    Quit,
}

impl MainMenuAction {
    pub(crate) fn variants() -> Vec<MainMenuAction> {
        vec![
            MainMenuAction::Staff,
            MainMenuAction::Student,
            MainMenuAction::Courses,
            MainMenuAction::TeacherLogin,
            MainMenuAction::Quit,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            MainMenuAction::Staff => "Staff Menu",
            MainMenuAction::Student => "Student Menu",
            MainMenuAction::Courses => "Courses Menu",
            MainMenuAction::TeacherLogin => "Teacher Login",
            MainMenuAction::Quit => "Quit",
        }
    }
}

impl fmt::Display for MainMenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}