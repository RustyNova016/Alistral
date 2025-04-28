use core::fmt;

use alistral_core::cli::colors::AlistralColors as _;
use color_eyre::owo_colors::OwoColorize;
use tracing::Event;
use tracing::Level;
use tracing::Metadata;
use tracing::Subscriber;
use tracing_indicatif::IndicatifLayer;
use tracing_indicatif::filter::IndicatifFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::filter;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt as _;
use tuillez::styles::COUNT_STYLE;

use crate::models::cli::Cli;

pub fn init_tracer(cli: &Cli) {
    let main_filter = filter::Targets::new()
        .with_target("alistral", Level::DEBUG)
        .with_target("alistral_core", Level::DEBUG)
        .with_target("musicbrainz_db_lite", Level::DEBUG)
        .with_target("interzic", Level::DEBUG);

    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(COUNT_STYLE.to_owned())
        .with_span_child_prefix_symbol("└─")
        .with_span_child_prefix_indent("  ");

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .event_format(PublicFormater);
    //.with_filter(filter);
    tracing_subscriber::registry()
        .with(cli.verbose.tracing_level_filter())
        .with(main_filter)
        .with(layer)
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

        write!(
            writer,
            "{}",
            match *metadata.level() {
                Level::ERROR => "[Error] ".red().to_string(),
                Level::WARN => "[Warn] ".yellow().to_string(),
                Level::INFO => "".to_string(),
                Level::DEBUG => "[Debug] ".cyan().to_string(),
                Level::TRACE => "[Trace] ".bright_black().to_string(),
            }
        )?;

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        // write!(
        //     writer,
        //     "{}",
        //     match *metadata.level() {
        //         Level::ERROR => fields.red().to_string(),
        //         Level::WARN => fields.yellow().to_string(),
        //         Level::INFO => fields,
        //         Level::DEBUG => fields.cyan().to_string(),
        //         Level::TRACE => fields.bright_black().to_string(),
        //     }
        // )?;

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
        "musicbrainz_db_lite" => "[MusicBrainz DB]".db_lite_purple(),
        "interzic" => "[Interzic]".interzic_turquoize(),
        _ => format!("[{}]", top_crate),
    };

    write!(writer, "{} ", content)
}
