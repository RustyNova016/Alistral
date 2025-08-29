use itertools::Itertools as _;
use musicbrainz_db_lite::User;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::listen::relations::UserListenDBRel;
use tracing::instrument;
use tuillez::pg_spinner;
use tuillez::tracing_utils::pg_future::PGFuture as _;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub type UserWithListensCollection = EntityWithListensCollection<User, ListenCollection>;

#[derive(Debug, Default)]
pub struct UserWithListensStrategy {}

impl UserWithListensStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

impl ListenSortingStrategy<User, ListenCollection> for UserWithListensStrategy {
    #[instrument(skip(self, client, data, listens), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<User, ListenCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        // If empty, early return
        if listens.is_empty() {
            return Ok(());
        }

        pg_spinner!("Compiling user listens data");
        let conn = &mut *client.musicbrainz_db.get_raw_connection().await?;

        let listen_refs = listens.iter().collect_vec();

        let joins = Listen::get_related_entity_bulk::<UserListenDBRel>(conn, &listen_refs)
            .pg_spinner("Loading user listens from cache...")
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
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<User, ListenCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, client, data, vec![listen]).await
    }
}
