use core::cmp::Ordering;
use core::fmt::Display;

use tuillez::OwoColorize as _;

pub struct PreviousTF<'l, T>
where
    T: Display + PartialOrd,
{
    current: &'l T,
    previous: &'l T,
    lower_is_better: bool,
}

impl<'l, T> PreviousTF<'l, T>
where
    T: Display + PartialOrd,
{
    pub fn get_arrow(&self) -> String {
        match self.previous.partial_cmp(self.current) {
            None => "-".bright_black().to_string(),
            Some(Ordering::Equal) => "-".bright_black().to_string(),
            Some(Ordering::Less) if self.lower_is_better => "▲".red().to_string(),
            Some(Ordering::Less) => "▲".green().to_string(),
            Some(Ordering::Greater) if self.lower_is_better => "▼".green().to_string(),
            Some(Ordering::Greater) => "▼".red().to_string(),
        }
    }
}

impl<'l, T> Display for PreviousTF<'l, T>
where
    T: Display + PartialOrd,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} - {}]", self.get_arrow(), self.previous)
    }
}
