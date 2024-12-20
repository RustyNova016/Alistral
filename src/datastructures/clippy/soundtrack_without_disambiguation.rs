use color_eyre::owo_colors::OwoColorize;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::models::clippy::lint_severity::LintSeverity;
use crate::models::clippy::MbClippyLint;
use crate::models::clippy::MbClippyLintLink;
use crate::utils::cli::display::WorkExt as _;

pub struct SoundtrackWithoutDisambiguationLint {
    work: Work,
}

impl MbClippyLint for SoundtrackWithoutDisambiguationLint {
    fn get_name() -> &'static str {
        "soundtrack_without_disambiguation"
    }

    async fn check(
        _conn: &mut sqlx::SqliteConnection,
        entity: &musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Work(work) = entity else {
            return Ok(None);
        };

        if work.work_type.as_ref().is_none_or(|t| t != "Soundtrack")
            || work.disambiguation.as_ref().is_some_and(|d| !d.is_empty())
        {
            return Ok(None);
        }

        Ok(Some(Self { work: work.clone() }))
    }

    async fn get_body(
        &self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!("Work \"{}\" has type `Soundtrack` and has no disambiguation. 
 -> Style guidelines for soundtrack works require the name of the original work to be in the disambiguation. 
    Additionally, if possible, a descriptive name, or the name of the episode"
        , self.work.pretty_format().await?))
    }

    async fn get_hints(
        &self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintHint>, crate::Error> {
        Ok(Vec::new())
    }

    async fn get_links(
        &self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintLink>, crate::Error> {
        Ok(vec![
            MbClippyLintLink {
                name: "Style Guidelines".truecolor(232, 133, 58).to_string(),
                url: "https://musicbrainz.org/doc/Style/Specific_types_of_releases/Soundtrack#Work"
                    .to_string(),
            },
            MbClippyLintLink {
                name: "Work editing".to_string(),
                url: format!("https://musicbrainz.org/work/{}/edit", self.work.mbid),
            },
        ])
    }

    fn get_severity(&self) -> crate::models::clippy::lint_severity::LintSeverity {
        LintSeverity::StyleIssue
    }
}
