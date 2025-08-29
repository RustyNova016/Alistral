use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID as _;
use musicbrainz_db_lite::Label;
use musicbrainz_db_lite::Release;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::release::relations::ReleaseLabelDBRel;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::database::fetching::releases::prefetch_releases;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::label::LabelWithReleases;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsStrategy;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub type LabelWithReleasesCollection =
    EntityWithListensCollection<Label, ReleaseWithRecordingsCollection>;

pub struct LabelWithReleasesStrategy<'l> {
    pub(super) client: &'l AlistralClient,
    release_strat: ReleaseWithRecordingsStrategy<'l>,
}

impl<'l> LabelWithReleasesStrategy<'l> {
    pub fn new(
        client: &'l AlistralClient,
        release_strat: ReleaseWithRecordingsStrategy<'l>,
    ) -> Self {
        Self {
            client,
            release_strat,
        }
    }
}

impl ListenSortingStrategy<Label, ReleaseWithRecordingsCollection>
    for LabelWithReleasesStrategy<'_>
{
    #[instrument(skip(self, client, data, listens), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Label, ReleaseWithRecordingsCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling label listens data");
        // Convert Releases
        let releases =
            ReleaseWithRecordingsCollection::from_listens(client, listens, &self.release_strat)
                .await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Prefetch the releases
        let recording_refs = releases.iter_entities().collect_vec();
        prefetch_releases(conn, self.client, &recording_refs).await?;

        // Load Releases
        let joins =
            Release::get_related_entity_bulk::<ReleaseLabelDBRel>(conn, &recording_refs).await?;

        // Convert releases groups
        let results = joins
            .into_many_to_many(recording_refs)
            .into_many_to_zero_right()
            .map_right(|right| {
                releases
                    .get_by_id(right.rowid())
                    .cloned()
                    .expect("Error while compiling. Got a release that isn't in the stats")
            });

        for (left, right) in results {
            // Insert the listens into the data structure
            if let Some(left) = left {
                data.insert_or_merge_entity_stats(LabelWithReleases {
                    entity: left,
                    listens: right.into(),
                });
            }
        }
        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Label, ReleaseWithRecordingsCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, client, data, vec![listen]).await
    }
}
