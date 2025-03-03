use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::database::fetching::recordings::prefetch_recordings_of_listens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
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

        // Get all the data from the DB
        let joins = Listen::get_recordings_as_batch(conn, user.id, listens.data).await?;

        // Convert into structs
        let mut out = Self::new();

        for (_, (listen, recordings)) in joins {
            for recording in recordings {
                out.insert_or_merge_listen(recording, listen.clone());
                //Span::current().pb_inc(1);
            }
        }

        Ok(out)
    }
}
