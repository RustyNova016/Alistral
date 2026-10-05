use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

pub(super) fn no_discoveries_with_previous(
    previous_discovered: &RecordingWithListensCollection,
) -> String {
    let mut out = "No new tracks have been discovered...".to_string();

    let track_count = previous_discovered.len();
    let listen_count: usize = previous_discovered
        .iter()
        .map(|rec| rec.listen_count())
        .sum();

    writeln!(
        out,
        "Which is less than the previous period's {} discoveries ({} total listens)",
        track_count.alistral_green(),
        listen_count.alistral_green()
    )
    .unwrap();

    out
}
