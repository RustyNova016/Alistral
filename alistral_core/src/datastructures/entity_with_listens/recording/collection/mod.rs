use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use musicbrainz_db_lite::utils::sqlx_utils::entity_relations::JoinRelation;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::pg_spinner;

use crate::database::fetching::recordings::prefetch_recordings_of_listens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::traits::FromListenCollection;
use crate::datastructures::listen_collection::ListenCollection;

pub type RecordingWithListensCollection = EntityWithListensCollection<Recording, ListenCollection>;

impl RecordingWithListensCollection {
    #[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        client: &crate::AlistralClient,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        // If empty, early return
        if listens.is_empty() {
            return Ok(Default::default());
        }
        pg_spinner!("Compiling recording listens data");

        // Prefetch the missing data
        let user_name = listens
            .first()
            .expect("At least one listen should be there")
            .user
            .clone();

        let user = User::find_by_name(conn, &user_name)
            .await?
            .ok_or(crate::Error::MissingUserError(user_name.clone()))?;

        prefetch_recordings_of_listens(conn, client, user.id, &listens.data).await?;

        Ok(Self::compile(
            Listen::get_recordings_as_batch(conn, user.id, &listens.data).await?,
            listens,
        ))
    }

    #[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
    fn compile(relations: Vec<JoinRelation<i64, Recording>>, listens: ListenCollection) -> Self {
        pg_counted!(relations.len(), "Loading listens data");
        let mut out = Self::new();

        relations
            .into_iter()
            .map(|join| {
                let listen = listens.iter().find(|l| l.id == join.original_id).unwrap();
                (listen.clone(), join.data)
            })
            .for_each(|(listen, recording)| {
                out.insert_or_merge_listen(recording, listen);
                pg_inc!()
            });

        out
    }
}

impl FromListenCollection for RecordingWithListensCollection {
    async fn from_listencollection(
        client: &crate::AlistralClient,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        Self::from_listencollection(
            client.musicbrainz_db.get_raw_connection().await?.as_mut(),
            client,
            listens,
        )
        .await
    }
}
