use core::error::Error;
use core::fmt::Display;
use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::fmt::Write;

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
            let text = err.to_string().replace("\n", "\n    ");
            writeln!(f, "    {text}")?;
            writeln!(f)?;
            let mut text = String::new();
            write!(text, "{err:#?}")?;
            let text = text.replace("\n", "\n    ");
            writeln!(f, "    Debug: {text}")?;
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

pub trait IntoFatal<T, E: Error> {
    fn unwrap_fatal(self) -> Result<T, FatalError>;
    fn expect_fatal(self, text: &str) -> Result<T, FatalError>;
}

impl<T, E: core::error::Error + 'static> IntoFatal<T, E> for Result<T, E> {
    fn unwrap_fatal(self) -> Result<T, FatalError> {
        self.map_err(|error| FatalError::new(error, None))
    }

    fn expect_fatal(self, text: &str) -> Result<T, FatalError> {
        self.map_err(|error| FatalError::new(error, Some(text.to_owned())))
    }
}

pub trait OptIntoFatal<T> {
    fn unwrap_fatal(self) -> Result<T, FatalError>;
    fn expect_fatal(self, text: &str) -> Result<T, FatalError>;
}

impl<T> OptIntoFatal<T> for Option<T> {
    fn unwrap_fatal(self) -> Result<T, FatalError> {
        self.ok_or_else(|| FatalError::new(crate::Error::UnwrapNone, None))
    }

    fn expect_fatal(self, text: &str) -> Result<T, FatalError> {
        self.ok_or_else(|| FatalError::new(crate::Error::UnwrapNone, Some(text.to_owned())))
    }
}
