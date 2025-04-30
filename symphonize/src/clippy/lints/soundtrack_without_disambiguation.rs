use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::work::Work;
use owo_colors::OwoColorize as _;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::clippy::lints::MusicbrainzLints;

pub struct SoundtrackWithoutDisambiguationLint {
    work: Work,
}

impl MbClippyLint for SoundtrackWithoutDisambiguationLint {
    fn get_name() -> &'static str {
        "soundtrack_without_disambiguation"
    }

    async fn check(
        _client: &SymphonyzeClient,
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
        _client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!("Work \"{}\" has type `Soundtrack` and has no disambiguation. 
 -> Style guidelines for soundtrack works require the name of the original work to be in the disambiguation. 
    Additionally, if possible, a descriptive name, or the name of the episode"
        , self.work.pretty_format().await?))
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        Ok(Vec::new())
    }

    async fn get_links(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
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

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::StyleIssue
    }
}

impl From<SoundtrackWithoutDisambiguationLint> for MusicbrainzLints {
    fn from(value: SoundtrackWithoutDisambiguationLint) -> Self {
        Self::SoundtrackWithoutDisambiguationLint(value)
    }
}
