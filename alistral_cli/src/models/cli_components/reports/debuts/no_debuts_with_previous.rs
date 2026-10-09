use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

pub(super) fn no_discoveries_with_previous<Ent, Lis>(
    previous_discovered: &EntityWithListensCollection<Ent, Lis>,
) -> String
where
    EntityWithListensCollection<Ent, Lis>: ListenCollectionReadable,
{
    let mut out = "No new tracks have been discovered...".to_string();

    let track_count = previous_discovered.entity_count();
    let listen_count = previous_discovered.listen_count();

    writeln!(
        out,
        "Which is less than the previous period's {} discoveries ({} total listens)",
        track_count.alistral_green(),
        listen_count.alistral_green()
    )
    .unwrap();

    out
}
