use std::fmt::Display;

pub struct Label(String);

impl Label {
    pub fn new(l: &str) -> Self {
        Label(l.to_string())
    }
}

pub trait ToLabel {
    fn to_label(&self) -> Label;
}

impl<T> ToLabel for T
where
    T: ToString,
{
    fn to_label(&self) -> Label {
        Label(self.to_string())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
