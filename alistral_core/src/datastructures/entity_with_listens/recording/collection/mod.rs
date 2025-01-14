use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;

use crate::database::fetching::recordings::prefetch_recordings_of_listens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;

pub type RecordingWithListensCollection = EntityWithListensCollection<Recording, ListenCollection>;

impl RecordingWithListensCollection {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        // If empty, early return
        if listens.is_empty() {
            return Ok(Default::default());
        }

        // Prefetch the missing data
        let user_name = listens
            .first()
            .expect("At least one listen should be there")
            .user
            .clone();

        let user = User::find_by_name(conn, &user_name)
            .await?
            .ok_or(crate::Error::MissingUserError(user_name.clone()))?;

        prefetch_recordings_of_listens(conn, user.id, &listens.data).await?;

        // Get all the data from the DB
        let joins = Listen::get_recordings_as_batch(conn, user.id, listens.data).await?;

        // Convert into structs
        let mut out = Self::new();

        for (_, (listen, recordings)) in joins {
            for recording in recordings {
                out.insert_or_merge_listen(recording, listen.clone());
            }
        }

        Ok(out)
    }
}
