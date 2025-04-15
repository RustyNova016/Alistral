pub mod error;
#[cfg(feature = "extensions")]
pub mod extensions;
pub mod fatal_error;
pub mod inquire_ext;
pub mod formatter;
pub mod macros;
pub mod styles;
pub mod utils;

pub use crate::error::Error;
pub use crate::styles::SPINNER_STYLE;

pub mod tracing_indicatif {
    pub use tracing_indicatif::*;
}

pub mod inquire {
    pub use inquire::*;
}

pub mod strum {
    pub use strum::*;
}
