use std::fmt::Display;

pub struct Markdown(pub(super) String);

impl Display for Markdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
