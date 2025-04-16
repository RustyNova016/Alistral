use core::fmt::Display;

pub struct MbClippyLintHint {
    text: String,
}

impl MbClippyLintHint {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Display for MbClippyLintHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "= Hint: {}", self.text)?;

        Ok(())
    }
}
