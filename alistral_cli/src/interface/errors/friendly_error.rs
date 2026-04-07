use alistral_core::cli::colors::AlistralColors;

/// A friendly panic should be displayed above. If you see this message, this means something went wrong and you should report it!
#[derive(Debug, snafu::Snafu, Clone)]
#[snafu(visibility(pub(crate)))]
pub struct FriendlyPanic {
    title: String,
    body: String,
}

impl FriendlyPanic {
    pub fn print_no_panic(&self) {
        println!(
            "\n\n{}\n\n{}\n",
            self.title.as_color_title((225, 0, 25)),
            self.body
        );
    }
}

pub trait GetFriendlyError {
    /// If the error is a friendly error, extract it
    fn get_friendly_error(&self) -> Option<FriendlyPanic>;
}

impl GetFriendlyError for FriendlyPanic {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        Some(self.clone())
    }
}
