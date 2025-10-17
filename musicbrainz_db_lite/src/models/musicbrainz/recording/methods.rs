use std::sync::Arc;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

use crate::DBClient;
use crate::FetchAsComplete;
use crate::models::musicbrainz::recording::Recording;

impl Recording {
    /// Return true if the recording is a remix.
    ///
    /// This is determined by checking if the recording got a remix relation with another recording, or a remixer relationship with an artist
    pub async fn is_remix(&self, conn: &mut sqlx::SqliteConnection) -> Result<bool, crate::Error> {
        let recording_rels = self.get_artist_relations(conn).await?; //TODO: Use tasks
        for relation in recording_rels {
            if relation.is_remixer_rel(self) {
                return Ok(true);
            }
        }

        let artist_relations = self.get_recording_relations(conn).await?;
        for relation in artist_relations {
            if relation.is_remix_of_rel(self) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn length_as_duration(&self) -> Option<Duration> {
        self.length.and_then(|length| {
            Duration::new(length.div_euclid(1000), length.rem_euclid(1000) as u32)
        })
    }

    pub async fn first_release_date_or_fetch(
        &self,
        client: Arc<DBClient>,
    ) -> Result<Option<DateTime<Utc>>, crate::Error> {
        if let Some(date) = self.first_release_date() {
            return Ok(Some(date));
        }

        // Not found? Fetch as whole
        let new = self.fetch_as_complete_as_task(client.clone()).await?;

        if let Some(date) = new.first_release_date() {
            return Ok(Some(date));
        }

        // Still not found? Find the earliest release
        let releases = new.get_releases(&client).await?;
        let mut min: Option<DateTime<Utc>> = None;
        for release in releases {
            let Some(date) = release.release_date() else {
                continue;
            };

            match min {
                Some(min_val) => min = Some(min_val.min(date)),
                None => min = Some(date),
            }
        }

        Ok(min)
    }
}
