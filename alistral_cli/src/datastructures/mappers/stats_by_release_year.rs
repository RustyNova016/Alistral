use alistral_core::AlistralClient;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use sequelles::ZeroToManyJoin;


/// Map listens to the the first release date of the recordings
#[derive(Debug, Default)]
pub struct StatsByReleaseYear {
    pub mapping: ZeroToManyJoin<i32, RecordingWithListens>,
}

impl StatsByReleaseYear {
    pub async fn insert_recording_stats(
        &mut self,
        client: &AlistralClient,
        recording: RecordingWithListens,
    ) -> Result<(), crate::Error> {
        let date = recording
            .entity()
            .first_release_date_or_fetch(client.musicbrainz_db.clone())
            .await?;

        self.mapping.push_entry(date.map(|date| date.year()), recording);

        Ok(())
    }

    pub async fn insert_recording_stats_collection(
        &mut self,
        client: &AlistralClient,
        recordings: RecordingWithListensCollection,
    ) -> Result<(), crate::Error> {
        for rec in recordings.into_iter() {
            self.insert_recording_stats(client, rec).await?
        }

        Ok(())
    }
}
