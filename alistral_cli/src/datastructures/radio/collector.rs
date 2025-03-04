use chrono::Duration;
use futures::Stream;
use futures::StreamExt;
use macon::Builder;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

#[derive(Debug, Builder)]
pub struct RadioCollector {
    duration: Option<Duration>,
    count: Option<u64>,
}

impl RadioCollector {
    pub async fn collect(
        &self,
        recordings: impl Stream<Item = Recording> + Unpin,
    ) -> Vec<Recording> {
        self.try_collect(recordings.map(Ok))
            .await
            .expect("All the items are `Ok`, so no potential error")
    }

    pub async fn try_collect(
        &self,
        mut recordings: impl Stream<Item = Result<Recording, crate::Error>> + Unpin,
    ) -> Result<Vec<Recording>, crate::Error> {
        let mut results = Vec::new();
        while let Some(recording) = recordings.next().await.transpose()? {
            results.push(recording.clone());

            if self.check_minimum_lenght(&results) {
                return Ok(results);
            }
        }

        Ok(results)
    }

    /// Return true if the lenght of the playlist satisfy the requested minimum time
    pub fn check_minimum_lenght(&self, playlist: &[Recording]) -> bool {
        let has_min_count = match self.count {
            Some(count) => playlist.len() as u64 >= count,
            None => self.duration.is_some() || playlist.len() as u64 >= 50,
        };

        if !has_min_count {
            return false;
        }

        match self.duration {
            Some(duration) => {
                playlist
                    .iter()
                    .map(|recording| recording.length_as_duration().unwrap_or_default())
                    .sum::<Duration>()
                    >= duration
            }
            None => true,
        }
    }
}
