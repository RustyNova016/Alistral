use std::sync::Arc;

use futures::StreamExt as _;
use futures::stream;
use streamies::TryStreamies as _;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::DBClient;
use crate::Label;
use crate::Release;

impl Release {
    /// Fetch all the labels for those releases.
    ///
    /// This isn't an optimized way to get the labels. Use this if you aren't sure if the labels are cached, and need a way to prefetch them
    #[instrument(skip(client, releases), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn get_or_fetch_labels_bulk(
        client: Arc<DBClient>,
        releases: Vec<Release>,
    ) -> Result<Vec<(Release, Vec<Label>)>, crate::Error> {
        pg_counted!(releases.len(), "Fetching release groups");

        let results = stream::iter(releases)
            .map(
                async |release| match release.get_labels_or_fetch(&client).await {
                    Ok(labels) => Ok((release, labels)),
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
