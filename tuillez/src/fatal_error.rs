use core::error::Error;
use core::fmt::Display;
use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;

use color_backtrace::btparse::deserialize;
use color_backtrace::BacktracePrinter;
use owo_colors::OwoColorize as _;

#[derive(Debug)]
pub struct FatalError {
    text: Option<String>,
    error: Option<Box<dyn Error>>,
    backtrace: Backtrace,
}

impl FatalError {
    pub fn new<T: Error + 'static>(error: T, text: Option<String>) -> Self {
        Self {
            error: Some(Box::new(error)),
            text,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn new_string(text: &str) -> Self {
        Self {
            error: None,
            text: Some(text.to_string()),
            backtrace: Backtrace::capture(),
        }
    }

    pub fn panic(self) -> ! {
        println!("{self}");
        std::process::exit(2)
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
        if let Some(text) = self.text.as_ref() {
            writeln!(f)?;
            writeln!(f, "🧨 Here's what went wrong:")?;
            let text = text.replace("\n", "\n    ");
            writeln!(f, "    {text}")?;
        }

        if let Some(err) = self.error.as_ref() {
            writeln!(f)?;
            writeln!(f, "🗒️  Here's the raw error data:")?;
            writeln!(f, "{err}")?;
            writeln!(f, "{err:#?}")?;
        }

        if self.backtrace.status() == BacktraceStatus::Captured {
            writeln!(f)?;
            let printer = BacktracePrinter::default();
            let bt = deserialize(&self.backtrace).unwrap();
            writeln!(f, "{}", printer.format_trace_to_string(&bt).unwrap())?;
        }

        Ok(())
    }
}

impl core::error::Error for FatalError {}