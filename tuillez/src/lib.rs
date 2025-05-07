pub mod error;
#[cfg(feature = "extensions")]
pub mod extensions;
pub mod fatal_error;
pub mod formatter;
pub mod macros;
pub mod styles;
pub mod tracing_utils;
pub mod utils;

pub use crate::error::Error;
pub use crate::styles::SPINNER_STYLE;

pub use owo_colors::OwoColorize;

pub mod tracing_indicatif {
    pub use tracing_indicatif::*;
}
