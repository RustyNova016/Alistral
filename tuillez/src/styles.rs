use std::sync::LazyLock;

use indicatif::ProgressState;
use tracing_indicatif::style::ProgressStyle;

pub static SPINNER_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{span_child_prefix}{spinner} [{msg}] | {elapsed_subsec}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
        .with_key("elapsed_subsec", elapsed_subsec)
});

pub static COUNT_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template(
        "{span_child_prefix}[{msg}]┫{wide_bar} {pos}/{len} ┃ {eta_precise} ┃ {elapsed_subsec}",
    )
    .unwrap()
    .with_key("elapsed_subsec", elapsed_subsec)
    .progress_chars(&format!("{}{}{}", "█", "┣", "━"))
});

fn elapsed_subsec(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let _ = writer.write_str(&format!("{seconds}.{sub_seconds}s"));
}
