use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum EditNewAction {
    Edit,
    New,
    Delete,
    Back,
}

impl EditNewAction {
    pub(crate) fn variants() -> Vec<EditNewAction> {
        vec![
            EditNewAction::Edit,
            EditNewAction::New,
            EditNewAction::Delete,
            EditNewAction::Back,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            EditNewAction::Edit => "Edit",
            EditNewAction::New => "New",
            EditNewAction::Delete => "Delete",
            EditNewAction::Back => "Back",
        }
    }
}

impl fmt::Display for EditNewAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}