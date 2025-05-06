use std::sync::Arc;

use futures::StreamExt;
use futures::stream;
use streamies::TryStreamies;

use crate::DBClient;
use crate::DBRelation;
use crate::Recording;
use crate::Release;

/// Recording (1:M) -> Releases
pub struct RecordingReleasesDBRel;

impl DBRelation<RecordingReleasesDBRel> for Recording {
    type ReturnedType = Release;

    fn get_join_statement() -> &'static str {
        "INNER JOIN tracks ON recordings.id = tracks.recording
        INNER JOIN medias ON tracks.media = medias.id
        INNER JOIN releases ON medias.`release` = releases.id"
    }
}

impl Recording {
    /// Get the releases of the recording that are harmony compatible
    pub async fn get_harmony_compatible_releases(
        &self,
        client: &Arc<DBClient>,
    ) -> Result<Vec<Release>, crate::Error> {
        let releases = self
            .get_related_entity_or_fetch_as_task::<RecordingReleasesDBRel>(client)
            .await?;

        stream::iter(releases)
            .filter_map(
                async |release| match release.is_harmony_compatible(client).await {
                    Err(err) => Some(Err(err)),
                    Ok(true) => Some(Ok(release)),
                    Ok(false) => None,
                },
            )
            .try_collect_vec()
            .await
    }
}
