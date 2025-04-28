use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use musicbrainz_db_lite::utils::sqlx_utils::entity_relations::JoinRelation;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::pg_spinner;

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
    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
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
        let user_name = listens
            .first()
            .expect("At least one listen should be there")
            .user
            .clone();

        let user = User::find_by_name(conn, &user_name)
            .await?
            .ok_or(crate::Error::MissingUserError(user_name.clone()))?;

        prefetch_recordings_of_listens(conn, self.client, user.id, &listens).await?;

        compile(
            data,
            Listen::get_recordings_as_batch(conn, user.id, &listens).await?,
            listens,
        );
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

#[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
fn compile(
    data: &mut EntityWithListensCollection<Recording, ListenCollection>,
    relations: Vec<JoinRelation<i64, Recording>>,
    listens: Vec<Listen>,
) {
    pg_counted!(relations.len(), "Loading listens data");

    let id_map = relations
        .into_iter()
        .map(|join| (join.original_id, join.data))
        .into_group_map();

    for (key, chunk) in &listens.into_iter().chunk_by(|l| id_map.get(&l.id).unwrap()) {
        data.insert_or_merge_listens(key.first().unwrap().clone(), chunk.collect_vec());
        pg_inc!()
    }
}
