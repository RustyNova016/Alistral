use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID as _;
use musicbrainz_db_lite::Recording;
use sequelles::JoinCollection;
use sequelles::JoinRelation;
use snafu::ResultExt;
use snafu::Snafu;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::release::ReleaseWithRecordings;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::traits::sorter::Linker;

impl Linker<ReleaseWithRecordingsCollection, RecordingWithListensCollection> for AlistralClient {
    type Returned = Result<(), ReleaseStatsLinkingError>;

    #[instrument(skip(self, data, item), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn link_entry(
        &self,
        data: &mut ReleaseWithRecordingsCollection,
        item: RecordingWithListensCollection,
    ) -> Self::Returned {
        pg_spinner!("Compiling release listen data");

        //TODO(perf): Don't clone the recordings, but keep `fetch_all_releases_bulk` Send
        let recordings = item.iter_entities().cloned().collect_vec();

        // Fetch
        let joins =
            Recording::fetch_all_releases_bulk(self.musicbrainz_db.clone(), recordings.clone())
                .await
                .context(ReleaseStatsLinkingSnafu)?;

        let mut join_col = JoinCollection::default();
        for (recording, artists) in joins {
            for artist in artists {
                join_col.push(JoinRelation::new(artist, recording.rowid()));
            }
        }

        let results = join_col
            .into_many_to_many(recordings)
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
                data.insert_or_merge_entity_stats(ReleaseWithRecordings {
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
        data: &mut ReleaseWithRecordingsCollection,
        items: Vec<RecordingWithListensCollection>,
    ) -> Self::Returned {
        for coll in items {
            self.link_entry(data, coll).await?
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Couldn't link the artists to the recording statistics"))]
pub struct ReleaseStatsLinkingError {
    source: musicbrainz_db_lite::Error,
        #[cfg(feature = "backtrace")]
    backtrace: snafu::Backtrace,
}
