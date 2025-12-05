use std::sync::Arc;

use futures::StreamExt as _;
use futures::stream;
use streamies::TryStreamies as _;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::DBClient;
use crate::Release;
use crate::ReleaseGroup;

impl Release {
    /// Fetch all the release groups that are part of the mutiple recording's credits.
    ///
    /// This isn't an optimized way to get the release groups. Use this if you aren't sure if the release groups are cached, and need a way to prefetch them
    #[instrument(skip(client, recordings), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn fetch_all_release_groups_bulk(
        client: Arc<DBClient>,
        recordings: Vec<Release>,
    ) -> Result<Vec<(Release, Vec<ReleaseGroup>)>, crate::Error> {
        pg_counted!(recordings.len(), "Fetching release groups");

        let results = stream::iter(recordings)
            .map(
                async |release| match release.get_release_group(&client).await {
                    Ok(releases) => Ok((release, releases)),
                    Err(crate::Error::NotFoundInUpstream(_)) => Ok((release, Vec::new())),
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
