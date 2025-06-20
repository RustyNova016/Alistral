use std::fmt;
use tracing::Event;
use tracing::Subscriber;
use tracing_subscriber::fmt::{
    FmtContext,
    format::{self, FormatEvent, FormatFields},
};
use tracing_subscriber::registry::LookupSpan;

pub(super) struct FileFormatter;

impl<S, N> FormatEvent<S, N> for FileFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "[{}] ", metadata.level())?;

        // === Write Level ===

        write!(&mut writer, "[{}]", metadata.target())?;

        // === Write Thread info ===

        write!(writer, "[{:0>2?} ", std::thread::current().id())?;

        let current_thread = std::thread::current();
        match current_thread.name() {
            Some(name) => {
                write!(writer, "{}] ", name)?;
            }
            // Close the braket
            None => {
                write!(writer, "] ")?;
            }
        }

        // === Filename ===
        if let Some(filename) = metadata.file() {
            write!(writer, "{}: ", filename)?;
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
