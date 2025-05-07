use tracing::info_span;
use tracing::instrument::Instrumented;
use tracing::Instrument;
use tracing_indicatif::span_ext::IndicatifSpanExt as _;

use crate::SPINNER_STYLE;

pub trait PGFuture: Instrument {
    /// Add a [`tracing_indicatif`] progress bar using the [`SPINNER_STYLE`] style.
    fn pg_spinner(self, msg: &str) -> Instrumented<Self> {
        let span = info_span!("Test");
        span.pb_set_message(msg);
        span.pb_set_style(&SPINNER_STYLE);
        self.instrument(span)
    }
}

impl<T: Instrument> PGFuture for T {}
