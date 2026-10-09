use core::fmt::Write as _;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::Datelike;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::reports::group_by_charts::group_by_report;

const MONTH_LABELS: [(u32, &'static str); 12] = [
    (1, "Jan"),
    (2, "Feb"),
    (3, "Mar"),
    (4, "Apr"),
    (5, "May"),
    (6, "Jun"),
    (7, "Jul"),
    (8, "Aug"),
    (9, "Sep"),
    (10, "Oct"),
    (11, "Nov"),
    (12, "Dec"),
];

pub fn group_by_month_report(
    current_listens: impl ListenCollectionReadable,
    previous_listens: impl ListenCollectionReadable,
) -> String {
    let mut out = String::new();

    writeln!(out, "{}", Heading1("Listens by months 📆")).unwrap();
    writeln!(out, "Here's all the listens of the {} period grouped by their months of listening, alongside the {} period", "current".alistral_green(), "previous".true_color_tup((18, 121, 198))).unwrap();
    writeln!(out).unwrap();

    write!(
        &mut out,
        "{}",
        group_by_report(
            &MONTH_LABELS,
            |listen| listen.listened_at_as_datetime().month(),
            current_listens,
            previous_listens,
        ),
    )
    .unwrap();

    out
}
