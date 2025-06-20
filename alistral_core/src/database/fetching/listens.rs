use macon::Builder;
use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::api::listenbrainz::listen::fetching::query::ListenFetchAPIQuery;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

#[derive(Builder)]
pub struct ListenFetchQuery {
    #[builder(Default=!)]
    user: String,

    fetch_recordings_redirects: bool,

    returns: ListenFetchQueryReturn,
}

impl ListenFetchQuery {
    pub async fn fetch(
        self,
        conn: &mut sqlx::SqliteConnection,
        client: &AlistralClient,
    ) -> Result<ListenCollection, crate::Error> {
        // Fetch the latest listens
        // ... If it's not in offline mode
        if !client.offline {
            let mut fetch = ListenFetchAPIQuery::incremental_fetch_user(
                &client.musicbrainz_db,
                self.user.clone(),
            )
            .await?;

            fetch.request_and_save(&client.musicbrainz_db).await?;
        }

        if self.fetch_recordings_redirects {
            Self::fetch_recordings_redirects(conn, client, &self.user).await?;
        }

        match self.returns {
            ListenFetchQueryReturn::Mapped => Ok(ListenCollection::new(
                Listen::get_mapped_listen_of_user(conn, &self.user).await?,
            )),
            ListenFetchQueryReturn::Unmapped => Ok(ListenCollection::new(
                Listen::get_unmapped_listen_of_user(conn, &self.user).await?,
            )),
            ListenFetchQueryReturn::None => Ok(ListenCollection::default()),
        }
    }

    #[instrument(skip(client, conn), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn fetch_recordings_redirects(
        conn: &mut sqlx::SqliteConnection,
        client: &AlistralClient,
        user: &str,
    ) -> Result<(), crate::Error> {
        let unfetched = Listen::get_unfetched_recordings_of_user(conn, user).await?;
        pg_counted!(unfetched.len(), "Fetching listen MBIDs");

        for id in unfetched {
            Recording::get_or_fetch(conn, &client.musicbrainz_db, &id).await?;
            pg_inc!();
        }

        Ok(())
    }

    pub async fn get_recordings_with_listens(
        client: &AlistralClient,
        user: String,
        strat: &RecordingWithListenStrategy<'_>,
    ) -> Result<RecordingWithListensCollection, crate::Error> {
        Self::get_entity_with_listens(client, user, strat).await
    }

    pub async fn get_entity_with_listens<Ent, Lis, S>(
        client: &AlistralClient,
        user: String,
        strat: &S,
    ) -> Result<EntityWithListensCollection<Ent, Lis>, crate::Error>
    where
        Ent: RowId,
        Lis: ListenCollectionReadable,
        S: ListenSortingStrategy<Ent, Lis>,
    {
        let query = Self {
            fetch_recordings_redirects: false,
            returns: ListenFetchQueryReturn::Mapped,
            user,
        };

        let listens = query
            .fetch(
                client.musicbrainz_db.get_raw_connection().await?.as_mut(),
                client,
            )
            .await?;

        EntityWithListensCollection::from_listencollection(listens, strat).await
    }
}

pub enum ListenFetchQueryReturn {
    Mapped,
    Unmapped,
    None,
}
