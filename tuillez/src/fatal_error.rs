use core::error::Error;
use core::fmt::Display;

use owo_colors::OwoColorize as _;

pub struct FatalError {
    text: Option<String>,
    error: Box<dyn Error>,
}

impl FatalError {
    pub fn new<T: Error + 'static>(error: T, text: Option<String>) -> Self {
        Self {
            error: Box::new(error),
            text,
        }
    }

    pub fn panic(self) -> ! {
        println!("{self}");
        let err: Result<(), Box<dyn Error>> = Err(self.error);
        #[expect(
            clippy::unnecessary_literal_unwrap,
            reason = "We need to print the trace, so we need to unwrap the error"
        )]
        err.unwrap();
        panic!();
    }
}

impl Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(
            f,
            "{}",
            " Unrecoverable Error \u{001b}[0K".bold().on_red().black()
        )?;
        writeln!(f)?;
        writeln!(f, "Something wrong happened, and the app couldn't recover.")?;
        writeln!(f)?;
        if let Some(text) = self.text.as_ref() {
            writeln!(f, "🧨 Here's what went wrong:")?;
            let text = text.replace("\n", "\n    ");
            writeln!(f, "    {text}")?;
            writeln!(f)?;
        }

        writeln!(f, "🗒️  Here's the raw error data:")?;

        Ok(())
    }
}
