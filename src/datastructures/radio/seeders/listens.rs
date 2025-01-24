use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::ListenCollection;
use derive_getters::Getters;
use itertools::Itertools;
use macon::Builder;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::database::listenbrainz::listens::fetch_latest_listens_of_user;

use super::SeederSettings;

/// A querry to generate a list of seed recording using the user's listens
#[derive(Debug, Builder, Getters, Clone)]
pub struct ListenSeeder {
    #[builder(Default=!)]
    pub username: String,

    #[builder(Default=!)]
    settings: SeederSettings,
}

impl ListenSeeder {
    pub async fn seed(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<RecordingWithListensCollection, crate::Error> {
        // Get the listens
        fetch_latest_listens_of_user(conn, &self.username).await?;

        let min_listened_at = self
            .settings
            .min_listened_at
            .map(|date| date.timestamp())
            .unwrap_or(0);

        let max_listened_at = self
            .settings
            .max_listened_at
            .map(|date| date.timestamp())
            .unwrap_or(i64::MAX);

        let listens: ListenCollection = sqlx::query_as!(
            Listen,
            "
            SELECT 
                listens.*
            FROM       
                users 
                INNER JOIN listens ON users.name = listens.user 
                INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
            WHERE
                -- Only for this user
                LOWER(listens.user) = LOWER(?)  
    
                -- Keep only mapped listens 
                AND msid_mapping.user = users.id 

                -- Minimum listened_at
                AND listens.listened_at >= ?

                -- Maximum listened_at
                AND listens.listened_at <= ?

            ORDER BY msid_mapping.recording_mbid",
            self.username,
            min_listened_at,
            max_listened_at,
        )
        .fetch_all(&mut *conn)
        .await?
        .into();

        let mut recordings =
            RecordingWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT, listens)
                .await?;
        let minimum_listens = self.get_minimum_listens(conn).await?;
        recordings.insert_or_merge(minimum_listens);

        Ok(recordings)
    }

    async fn get_minimum_listens(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<RecordingWithListensCollection, crate::Error> {
        // Early exit if no minimums
        if self.settings.min_listen_per_recording == 0 {
            return Ok(Default::default());
        }

        // TODO: Elegant SQL query that prevents manual processing
        let after_date = self
            .settings
            .min_listened_at
            .map(|date| date.timestamp())
            .unwrap_or(0);
        let listens: ListenCollection = sqlx::query_as!(
            Listen,
            "
            SELECT 
                listens.*
            FROM       
                users 
                INNER JOIN listens ON users.name = listens.user 
                INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
            WHERE
                -- Only for this user
                LOWER(listens.user) = LOWER(?)  
    
                -- Keep only mapped listens 
                AND msid_mapping.user = users.id
                
                -- After date
                AND listens.listened_at >= ?",
            self.username,
            after_date
        )
        .fetch_all(&mut *conn)
        .await?
        .into();

        let mapped =
            RecordingWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT, listens)
                .await?
                .into_iter()
                .map(|r| {
                    // Extract the last X listens from the collection
                    let listens = r
                        .listens()
                        .get_latest_listens(self.settings.min_listen_per_recording as usize);
                    RecordingWithListens::new(r.recording().clone(), listens)
                })
                .collect_vec();

        Ok(mapped.into())
    }
}
