use std::sync::LazyLock;

use indicatif::ProgressStyle;
use tracing_indicatif::indicatif_println;

pub static SPINNER_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{span_child_prefix}{spinner} [{msg}]")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
});

#[macro_export]
macro_rules! pg_spinner {
    ($($arg:tt)*) => {
        {
            use tracing::Span;
            use tracing_indicatif::span_ext::IndicatifSpanExt as _;
            use indicatif::ProgressStyle;

            Span::current().pb_set_message(&format!($($arg)*));
            Span::current().pb_set_style(
                &ProgressStyle::with_template("{span_child_prefix}{spinner} [{msg}]")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            );
        }
    }
}

#[macro_export]
macro_rules! pg_counted {
    ($length: expr, $($arg:tt)*) => {{
        use tracing::Span;

        Span::current().pb_set_length($length as u64);
        Span::current().pb_set_message(&format!($($arg)*));
    }};
}
