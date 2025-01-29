use core::fmt;

use alistral_core::cli::colors::AlistralColors as _;
use color_eyre::owo_colors::OwoColorize;
use indicatif::ProgressState;
use indicatif::ProgressStyle;
use tracing::Event;
use tracing::Level;
use tracing::Metadata;
use tracing::Subscriber;
use tracing_indicatif::filter::IndicatifFilter;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::filter;
use tracing_subscriber::fmt::format;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::Layer;

pub fn init_tracer() {
    let main_filter = filter::Targets::new()
        .with_target("alistral", Level::DEBUG)
        .with_target("interzic", Level::DEBUG);

    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(
            ProgressStyle::with_template(
                "{span_child_prefix}[{msg}] {wide_bar} {pos}/{len} | {eta_precise} | {elapsed_subsec}",
            )
            .unwrap()
            .with_key("elapsed_subsec", elapsed_subsec),
        )
        .with_span_child_prefix_symbol("┗ ")
        .with_span_child_prefix_indent("  ");

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .event_format(PublicFormater);
    //.with_filter(filter);
    tracing_subscriber::registry()
        .with(layer)
        .with(main_filter)
        .with(indicatif_layer.with_filter(IndicatifFilter::new(false)))
        .init();
}

struct PublicFormater;

impl<S, N> FormatEvent<S, N> for PublicFormater
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

        get_domain(&mut writer, metadata)?;

        //write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                // write!(writer, "{}", span.name())?;

                // // `FormattedFields` is a formatted representation of the span's
                // // fields, which is stored in its extensions by the `fmt` layer's
                // // `new_span` method. The fields will have been formatted
                // // by the same field formatter that's provided to the event
                // // formatter in the `FmtContext`.
                // let ext = span.extensions();
                // let fields = &ext
                //     .get::<FormattedFields<N>>()
                //     .expect("will never be `None`");

                // // Skip formatting the fields if the span had no fields.
                // if !fields.is_empty() {
                //     write!(writer, "{{{}}}", fields)?;
                // }
                //write!(writer, "§: ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

fn get_domain(writer: &mut format::Writer<'_>, metadata: &Metadata<'static>) -> fmt::Result {
    let top_crate = metadata
        .module_path()
        .and_then(|path| path.split("::").next());
    let Some(top_crate) = top_crate else {
        return Ok(());
    };

    let content = match top_crate {
        "alistral" => "[Alistral]".alistral_green(),
        "alistral_core" => "[Alistral]".alistral_green(),
        "interzic" => "[Interzic]".interzic_red(),
        _ => format!("[{}]", top_crate),
    };

    write!(writer, "{} ", content)
}

fn elapsed_subsec(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let _ = writer.write_str(&format!("{}.{}s", seconds, sub_seconds));
}
