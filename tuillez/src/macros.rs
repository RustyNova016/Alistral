#[macro_export]
macro_rules! pg_spinner {
    ($($arg:tt)*) => {
        {
            use tracing::Span;
            use tuillez::tracing_indicatif::span_ext::IndicatifSpanExt as _;
            use tuillez::SPINNER_STYLE;

            Span::current().pb_set_message(&format!($($arg)*));
            Span::current().pb_set_style(
                &SPINNER_STYLE
            );
        }
    }
}

#[macro_export]
macro_rules! pg_counted {
    ($length: expr, $($arg:tt)*) => {{
        use tracing::Span;
        use tuillez::tracing_indicatif::span_ext::IndicatifSpanExt as _;

        Span::current().pb_set_length($length as u64);
        Span::current().pb_set_message(&format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! pg_inc {
    () => {{
        use tracing::Span;
        use tuillez::tracing_indicatif::span_ext::IndicatifSpanExt as _;

        Span::current().pb_inc(1);
    }};

    ($inc: expr) => {{
        use tracing::Span;

        Span::current().pb_inc($inc);
    }};
}
