use std::sync::Arc;

use futures::StreamExt;
use futures::TryStreamExt;
use futures::stream;
use streamies::TryStreamies;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::Artist;
use crate::DBClient;
use crate::GetOrFetch;
use crate::HasArtistCredits as _;
use crate::Recording;

impl Recording {
    /// Fetch all the artists that are part of the recording's credits.
    ///
    /// This isn't an optimized way to get the artists. Use this if you aren't sure if the artists are cached
    pub async fn fetch_all_artists_from_credits(
        &self,
        client: Arc<DBClient>,
    ) -> Result<Vec<Artist>, crate::Error> {
        let credits = self
            .get_artist_credits_or_fetch_tasked(client.clone())
            .await?;

        let results = stream::iter(credits.1)
            .map(async |credit| {
                Artist::get_or_fetch_as_task(client.clone(), &credit.artist_gid).await
            })
            .buffer_unordered(8)
            .try_filter_map(async |artist| Ok(artist))
            .try_collect_vec()
            .await?;

        Ok(results)
    }

    /// Fetch all the artists that are part of the mutiple recording's credits.
    ///
    /// This isn't an optimized way to get the artists. Use this if you aren't sure if the artists are cached, and need a way to prefetch them
    #[instrument(skip(client, recordings), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn fetch_all_artists_from_credits_bulk<'recording>(
        client: Arc<DBClient>,
        recordings: Vec<Recording>,
    ) -> Result<Vec<(Recording, Vec<Artist>)>, crate::Error> {
        pg_counted!(recordings.len(), "Fetching artists");

        let results = stream::iter(recordings)
            .map(async |recording| {
                let artists = recording
                    .fetch_all_artists_from_credits(client.clone())
                    .await;

                match artists {
                    Ok(artists) => Ok((recording, artists)),
                    Err(err) => Err(err),
                }
            })
            .buffer_unordered(8)
            .inspect(|_| pg_inc!())
            .try_collect_vec()
            .await?;

        Ok(results)
    }
}
