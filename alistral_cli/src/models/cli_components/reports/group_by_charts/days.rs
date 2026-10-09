use core::fmt::Write as _;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::Datelike;

use crate::datastructures::cli_formating::title::Heading1;
use crate::models::cli_components::reports::group_by_charts::group_by_report;

const DAY_LABELS: [(u32, &'static str); 7] = [
    (1, "Mon"),
    (2, "Tue"),
    (3, "Wed"),
    (4, "Thu"),
    (5, "Fri"),
    (6, "Sat"),
    (7, "Sun"),
];

pub fn group_by_day_report(
    current_listens: impl ListenCollectionReadable,
    previous_listens: impl ListenCollectionReadable,
) -> String {
    let mut out = String::new();

    writeln!(out, "{}", Heading1("Listens by weekdays 📆")).unwrap();
    writeln!(out, "Here's all the listens of the {} period grouped by the day of the week when they were listened, alongside the {} period", "current".alistral_green(), "previous".true_color_tup((18, 121, 198))).unwrap();
    writeln!(out).unwrap();

    write!(
        &mut out,
        "{}",
        group_by_report(
            &DAY_LABELS,
            |listen| listen
                .listened_at_as_datetime()
                .weekday()
                .number_from_monday(),
            current_listens,
            previous_listens,
        ),
    )
    .unwrap();

    out
}
