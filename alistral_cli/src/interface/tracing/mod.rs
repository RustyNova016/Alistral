pub mod file_format;
use core::fmt;

use alistral_core::cli::colors::AlistralColors as _;
use chrono::Local;
use tracing::Event;
use tracing::Level;
use tracing::Metadata;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::Rotation;
use tracing_indicatif::IndicatifLayer;
use tracing_indicatif::filter::IndicatifFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::filter;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt as _;
use tuillez::OwoColorize as _;
use tuillez::styles::COUNT_STYLE;

use crate::interface::tracing::file_format::FileFormatter;
use crate::models::cli::Cli;
use crate::utils::constants::paths::LOG_DIR;

pub fn init_tracer(cli: &Cli) -> WorkerGuard {
    // === Console and indicatif ===

    let filter = filter::Targets::new()
        .with_target("alistral", Level::DEBUG)
        .with_target("alistral_core", Level::DEBUG)
        .with_target("musicbrainz_db_lite", Level::DEBUG)
        .with_target("interzic", Level::DEBUG)
        .with_target("symphonize", Level::DEBUG);

    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(COUNT_STYLE.to_owned())
        .with_span_child_prefix_symbol("└─")
        .with_span_child_prefix_indent("  ");

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .event_format(PublicFormater)
        .with_filter(filter)
        .with_filter(cli.verbose.tracing_level_filter());

    // === Registry ===

    let (log_file, guard) = get_logging_layer();
    tracing_subscriber::registry()
        //.with(console_subscriber::spawn())
        .with(console_layer)
        .with(indicatif_layer.with_filter(IndicatifFilter::new(false)))
        .with(log_file)
        .init();

    guard
}

fn get_logging_layer<S>() -> (impl Layer<S>, WorkerGuard)
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    let now = Local::now();

    let file_appender = tracing_appender::rolling::Builder::new()
        .filename_suffix("log")
        .rotation(Rotation::NEVER)
        .max_log_files(100)
        .filename_prefix(format!("log-{}", now.format("%Y-%m-%d_%H:%M:%S")))
        .build(&*LOG_DIR)
        .expect("Couldn't create log file appender");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = filter::Targets::new()
        .with_target("alistral", Level::DEBUG)
        .with_target("alistral_core", Level::DEBUG)
        .with_target("musicbrainz_db_lite", Level::DEBUG)
        .with_target("interzic", Level::DEBUG)
        .with_target("symphonize", Level::DEBUG)
        .with_target("musicbrainz_rs_nova", Level::DEBUG)
        .with_target("musicbrainz_rs", Level::DEBUG)
        .with_target("sqlx_core", Level::INFO);

    (
        layer()
            // Use a more compact, abbreviated log format
            //.compact()
            .with_ansi(false)
            // Display source code file paths
            .with_file(true)
            // Display source code line numbers
            .with_line_number(true)
            // Display the thread ID an event was recorded on
            .with_thread_ids(true)
            // Don't display the event's target (module path)
            .with_target(false)
            .with_writer(non_blocking)
            .event_format(FileFormatter)
            .with_filter(filter),
        guard,
    )
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
        _ => format!("[{top_crate}]"),
    };

    write!(writer, "{content} ")
}
