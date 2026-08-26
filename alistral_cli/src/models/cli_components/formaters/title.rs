use core::fmt::Display;
use tuillez::OwoColorize as _;

pub struct Title(pub String);

impl Title {
    pub fn new(data: impl Display) -> Self {
        Self(data.to_string())
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let inner = format!(" {} ", self.0);
        write!(f, "{}", inner.on_green().black().bold())
    }
}
