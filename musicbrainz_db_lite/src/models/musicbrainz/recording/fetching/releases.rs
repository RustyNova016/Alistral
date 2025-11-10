use std::sync::Arc;

use futures::StreamExt as _;
use futures::stream;
use streamies::TryStreamies as _;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::DBClient;
use crate::Recording;
use crate::Release;

impl Recording {
    /// Fetch all the artists that are part of the mutiple recording's credits.
    ///
    /// This isn't an optimized way to get the artists. Use this if you aren't sure if the artists are cached, and need a way to prefetch them
    #[instrument(skip(client, recordings), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn fetch_all_releases_bulk(
        client: Arc<DBClient>,
        recordings: Vec<Recording>,
    ) -> Result<Vec<(Recording, Vec<Release>)>, crate::Error> {
        pg_counted!(recordings.len(), "Fetching releases");

        let results = stream::iter(recordings)
            .map(
                async |recording| match recording.get_releases(&client).await {
                    Ok(releases) => Ok((recording, releases)),
                    Err(err) => Err(err),
                },
            )
            .buffer_unordered(8)
            .inspect(|_| pg_inc!())
            .try_collect_vec()
            .await?;

        Ok(results)
    }
}
