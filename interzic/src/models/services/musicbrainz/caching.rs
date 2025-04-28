use futures::TryStreamExt;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::InterzicClient;
use crate::models::messy_recording::MessyRecording;
use crate::models::services::musicbrainz::Musicbrainz;

impl Musicbrainz {
    #[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn reload_urls(client: &InterzicClient) -> Result<(), crate::Error> {
        let recordings: Vec<MessyRecording> =
            MessyRecording::iter_recordings_with_mbids(&client.database_client)
                .try_collect()
                .await?;
        pg_counted!(recordings.len(), "Reloading recordings");

        for recording in recordings {
            //TODO: Integrate MBDBLite to cache calls
            Self::fetch_and_save_urls(client, &recording).await?;
            pg_inc!();
        }

        Ok(())
    }
}
