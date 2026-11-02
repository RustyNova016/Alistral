use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzEntity;

pub(super) fn no_comebacks_with_previous<Ent, Lis>(
    previous_discovered: &EntityWithListensCollection<Ent, Lis>,
) -> String
where
    Ent: MusicbrainzEntity,
    EntityWithListensCollection<Ent, Lis>: ListenCollectionReadable,
{
    let mut out = format!("No {} have made their comeback...", Ent::entity_name());

    let track_count = previous_discovered.entity_count();
    let listen_count = previous_discovered.listen_count();

    writeln!(
        out,
        "Which is less than the previous period's {} comebacks ({} total listens)",
        track_count.alistral_green(),
        listen_count.alistral_green()
    )
    .unwrap();

    out
}
