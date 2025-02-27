pub mod error;
#[cfg(feature = "extensions")]
pub mod extensions;
pub mod fatal_error;
pub mod macros;
pub mod styles;
pub mod utils;

pub use crate::error::Error;
pub use crate::styles::SPINNER_STYLE;
