use chrono::Duration;

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
}
