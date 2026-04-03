use alistral_core::cli::colors::AlistralColors;
use tracing::error;

pub fn friendly_panic(title: &str, body: &str) -> Result<(), crate::Error> {
    println!("\n\n{}\n\n{body}\n", title.as_color_title((225, 0, 25)));

    Err(crate::Error::FriendlyPanic)
}
