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
use crate::datastructures::entity_with_listens::label::LabelWithReleases;
use crate::datastructures::entity_with_listens::label::collection::LabelWithReleasesCollection;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::traits::sorter::Linker;

impl Linker<LabelWithReleasesCollection, ReleaseWithRecordingsCollection> for AlistralClient {
    type Returned = Result<(), LabelStatsLinkingError>;

    #[instrument(skip(self, data, item), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn link_entry(
        &self,
        data: &mut LabelWithReleasesCollection,
        item: ReleaseWithRecordingsCollection,
    ) -> Self::Returned {
        pg_spinner!("Compiling label listen data");

        //TODO(perf): Don't clone the recordings, but keep `fetch_all_releases_bulk` Send
        let releases = item.iter_entities().cloned().collect_vec();

        // Fetch
        let joins =
            Release::get_or_fetch_labels_bulk(self.musicbrainz_db.clone(), releases.clone())
                .await
                .context(LabelStatsLinkingSnafu)?;

        let mut join_col = JoinCollection::default();
        for (recording, artists) in joins {
            for artist in artists {
                join_col.push(JoinRelation::new(artist, recording.rowid()));
            }
        }

        let results = join_col.into_many_to_many(releases);

        // Convert artists
        for (release, label) in results.relations() {
            // Insert the listens into the data structure

            data.insert_or_merge_entity_stats(LabelWithReleases {
                entity: label.to_owned(),
                listens: item.get_by_id(release.id).unwrap().to_owned().into(),
            });
        }
        Ok(())
    }

    #[instrument(skip(self, data, items), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn link_entry_batch(
        &self,
        data: &mut LabelWithReleasesCollection,
        items: Vec<ReleaseWithRecordingsCollection>,
    ) -> Self::Returned {
        for coll in items {
            self.link_entry(data, coll).await?
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("Couldn't link the labels to the recording statistics"))]
pub struct LabelStatsLinkingError {
    backtrace: Backtrace,
    source: musicbrainz_db_lite::Error,
}
