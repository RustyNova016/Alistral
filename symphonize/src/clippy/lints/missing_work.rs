use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};
use musicbrainz_db_lite::models::shared_traits::db_relation::RecordingWorkDBRel;
use tuillez::formatter::FormatWithAsync;

use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;

use crate::SymphonyzeClient;
use crate::clippy::lints::MusicbrainzLints;
use crate::utils::formater;

pub struct MissingWorkLint {
    recording: Recording,
}

impl MbClippyLint for MissingWorkLint {
    fn get_name() -> &'static str {
        "missing_recording_work"
    }

    async fn prefetch_entities(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<(), crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(());
        };

        recording
            .get_related_entity_or_fetch_as_task::<RecordingWorkDBRel>(&client.mb_database)
            .await?;

        Ok(())
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Vec<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(Vec::new());
        };

        let work = recording
            .get_related_entity::<RecordingWorkDBRel>(
                &mut *client.mb_database.get_raw_connection().await?,
            )
            .await?;

        if !work.is_empty() {
            return Ok(Vec::new());
        }

        let missing_work_lint = Self {
            recording: recording.clone(),
        };

        Ok(vec![missing_work_lint])
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!("Recording \"{}\" has no associated works
-> Most recordings should have a work associated to them. Please check if a work exists for a recording and add it / create it"
, self.recording.format_with_async(&formater(client)).await?))
    }

    async fn get_links(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();
        let conn = &mut client.mb_database.get_raw_connection().await?;
        let releases = self
            .recording
            .get_releases_or_fetch(conn, &client.mb_database)
            .await?;

        out.push(MbClippyLintLink {
            name: "Recording".to_string(),
            url: format!("https://musicbrainz.org/recording/{}", self.recording.mbid),
        });

        if let Some(release) = releases.first() {
            out.push(MbClippyLintLink {
                name: "Release relations".to_string(),
                url: format!(
                    "https://musicbrainz.org/release/{}/edit-relationships",
                    release.mbid
                ),
            });
        }

        Ok(out)
    }

    #[expect(clippy::vec_init_then_push)]
    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let mut hints = Vec::new();

        hints.push(MbClippyLintHint::new("Recordings of more spontaneous actions like improvisations and field recordings generally don't need works".to_string()));

        // TODO: #526 missing_recording_work: add hint that remixes uses the same work
        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingData
    }
}

impl From<MissingWorkLint> for MusicbrainzLints {
    fn from(value: MissingWorkLint) -> Self {
        Self::MissingWorkLint(value)
    }
}

#[cfg(test)]
mod test {

    use musicbrainz_db_lite::Recording;

    use crate::SymphonyzeClient;
    use crate::clippy::lints::missing_work::MissingWorkLint;
    use crate::testing::should_trigger_lint;
    use crate::testing::shouldnt_trigger_lint;
    use crate::testing::test_name;

    #[tokio::test]
    async fn should_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("missing_work.json").await;

        should_trigger_lint::<MissingWorkLint, Recording>(
            &client,
            "51feae17-26bd-4aa9-bfaa-ef408df58293",
        )
        .await;
    }

    #[tokio::test]
    async fn shouldnt_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("missing_work.json").await;

        shouldnt_trigger_lint::<MissingWorkLint, Recording>(
            &client,
            "e8454ea9-b850-46b0-b8d8-22d024095819",
        )
        .await;
    }
}
