use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::listen::relations::recording::ListenRecordingDBRel;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use tracing::instrument;
use tuillez::pg_spinner;
use tuillez::tracing_utils::pg_future::PGFuture;

use crate::AlistralClient;
use crate::database::fetching::recordings::prefetch_recordings_of_listens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::RecordingWithListens;
use crate::datastructures::entity_with_listens::traits::IterRecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub type RecordingWithListensCollection = EntityWithListensCollection<Recording, ListenCollection>;

impl IterRecordingWithListens for RecordingWithListensCollection {
    fn iter_recording_with_listens(&self) -> impl Iterator<Item = &RecordingWithListens> {
        self.0.values()
    }
}

pub struct RecordingWithListenStrategy<'l> {
    pub(super) client: &'l AlistralClient,
}

impl<'l> RecordingWithListenStrategy<'l> {
    pub fn new(client: &'l AlistralClient) -> Self {
        Self { client }
    }
}

impl ListenSortingStrategy<Recording, ListenCollection> for RecordingWithListenStrategy<'_> {
    #[instrument(skip(self, data, listens), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        data: &mut EntityWithListensCollection<Recording, ListenCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        // If empty, early return
        if listens.is_empty() {
            return Ok(());
        }

        pg_spinner!("Compiling recording listens data");
        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Prefetch the missing data
        // TODO: Make it user agnostic
        let user_name = listens
            .first()
            .expect("At least one listen should be there")
            .user
            .clone();

        let user = User::find_by_name(conn, &user_name)
            .await?
            .ok_or(crate::Error::MissingUserError(user_name.clone()))?;

        prefetch_recordings_of_listens(conn, self.client, user.id, &listens).await?;
        let listen_refs = listens.iter().collect_vec();

        let joins = Listen::get_related_entity_bulk::<ListenRecordingDBRel>(conn, &listen_refs)
            .pg_spinner("Loading recordings from cache...")
            .await?;

        for (recording, listens) in joins.into_many_to_zero(listens).invert() {
            // Insert the listens into the data structure
            if let Some(recording) = recording {
                data.insert_or_merge_listens(recording, listens);
            }
        }

        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        data: &mut EntityWithListensCollection<Recording, ListenCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, data, vec![listen]).await
    }
}
