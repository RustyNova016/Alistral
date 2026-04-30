use core::fmt::Display;

use alistral_core::cli::colors::AlistralColors as _;
use tuillez::OwoColorize as _;

pub struct Heading1<T: Display>(pub T);

impl<T> Display for Heading1<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "{}",
            format!("\n {} ", self.0)
                .on_alistral_dark_green()
                .black()
                .bold()
        )
    }
}
