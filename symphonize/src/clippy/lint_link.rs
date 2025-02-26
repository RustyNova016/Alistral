use core::fmt::Display;

use owo_colors::OwoColorize as _;

pub struct MbClippyLintLink {
    pub name: String,
    pub url: String,
}

impl Display for MbClippyLintLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.url.blue())?;

        Ok(())
    }
}
