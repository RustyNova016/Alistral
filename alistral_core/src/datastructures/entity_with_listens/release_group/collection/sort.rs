use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID as _;
use musicbrainz_db_lite::Release;
use sequelles::JoinCollection;
use sequelles::JoinRelation;
use snafu::Backtrace;
use snafu::ResultExt;
use snafu::Snafu;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::datastructures::entity_with_listens::release_group::ReleaseGroupWithReleases;
use crate::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesCollection;
use crate::traits::sorter::Linker;

impl Linker<ReleaseGroupWithReleasesCollection, ReleaseWithRecordingsCollection>
    for AlistralClient
{
    type Returned = Result<(), ReleaseGroupStatsLinkingError>;

    #[instrument(skip(self, data, item), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn link_entry(
        &self,
        data: &mut ReleaseGroupWithReleasesCollection,
        item: ReleaseWithRecordingsCollection,
    ) -> Self::Returned {
        pg_spinner!("Compiling artist listen data");

        //TODO(perf): Don't clone the recordings, but keep `fetch_all_releases_bulk` Send
        let releases = item.iter_entities().cloned().collect_vec();

        // Fetch
        let joins =
            Release::fetch_all_release_groups_bulk(self.musicbrainz_db.clone(), releases.clone())
                .await
                .context(ReleaseGroupStatsLinkingSnafu)?;

        let mut join_col = JoinCollection::default();
        for (recording, artists) in joins {
            for artist in artists {
                join_col.push(JoinRelation::new(artist, recording.rowid()));
            }
        }

        let results = join_col
            .into_many_to_many(releases)
            .into_many_to_zero_right()
            .map_right(|right| {
                item.get_by_id(right.rowid())
                    .cloned()
                    .expect("Error while compiling. Got a recording that isn't in the stats")
            });

        // Convert artists
        for (left, right) in results {
            // Insert the listens into the data structure
            if let Some(left) = left {
                data.insert_or_merge_entity_stats(ReleaseGroupWithReleases {
                    entity: left,
                    listens: right.into(),
                });
            }
        }
        Ok(())
    }

    #[instrument(skip(self, data, items), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn link_entry_batch(
        &self,
        data: &mut ReleaseGroupWithReleasesCollection,
        items: Vec<ReleaseWithRecordingsCollection>,
    ) -> Self::Returned {
        for coll in items {
            self.link_entry(data, coll).await?
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Couldn't link the artists to the recording statistics"))]
pub struct ReleaseGroupStatsLinkingError {
    backtrace: Backtrace,
    source: musicbrainz_db_lite::Error,
}
