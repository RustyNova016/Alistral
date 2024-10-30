use chrono::DateTime;
use chrono::Utc;
use derive_getters::Getters;
use itertools::Itertools;
use macon::Builder;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::database::get_db_client;
use crate::database::listenbrainz::listens::fetch_latest_listens_of_user;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;

/// A querry to generate a list of seed recording using the user's listens
#[derive(Debug, Builder, Getters)]
pub struct ListenSeeder {
    #[builder(Default=!)]
    username: String,

    // Only take the listens that happened after this date
    #[builder(Option=!)]
    after_date: Option<DateTime<Utc>>,

    // Add the latest X listens of each recording to the result, ignoring [`after_date`](ListenSeeder::after_date)
    minimum_listen: u64,
}

impl ListenSeeder {
    pub async fn seed(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<RecordingWithListens>, crate::Error> {
        // Get the listens
        fetch_latest_listens_of_user(get_db_client().await.as_welds_client(), &self.username)
            .await?;

        let after_date = self.after_date.map(|date| date.timestamp()).unwrap_or(0);
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

                -- Handle Timerange
                AND listens.listened_at >= ?

            ORDER BY msid_mapping.recording_mbid",
            self.username,
            after_date
        )
        .fetch_all(&mut *conn)
        .await?
        .into();

        let mut mapped_listens = RecordingWithListens::from_listencollection(conn, listens)
            .await?
            .into_values()
            .collect_vec();

        let minimum_listens = self.get_minimum_listens(conn).await?;

        for mapping in &mut mapped_listens {
            for other_mapping in &minimum_listens {
                if mapping.recording().id == other_mapping.recording().id {
                    mapping.merge(other_mapping.clone());
                }
            }
        }

        Ok(mapped_listens)
    }

    async fn get_minimum_listens(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<RecordingWithListens>, crate::Error> {
        // Early exit if no minimums
        if self.minimum_listen == 0 {
            return Ok(Vec::new());
        }

        // TODO: Elegant SQL query that prevents manual processing
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
                AND msid_mapping.user = users.id",
            self.username
        )
        .fetch_all(&mut *conn)
        .await?
        .into();

        let mapped = RecordingWithListens::from_listencollection(conn, listens)
            .await?
            .into_values()
            .map(|r| {
                // Extract the last X listens from the collection
                let listens = r.listens().get_latest_listens(self.minimum_listen as usize);
                RecordingWithListens::new(r.recording().clone(), listens)
            })
            .collect_vec();

        Ok(mapped)
    }
}
