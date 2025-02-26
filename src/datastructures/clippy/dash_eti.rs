use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use regex::Regex;

use crate::models::clippy::lint_severity::LintSeverity;
use crate::models::clippy::MbClippyLint;
use crate::models::clippy::MbClippyLintHint;
use crate::models::clippy::MbClippyLintLink;
use crate::utils::cli::display::RecordingExt as _;

pub struct DashETILint {
    recording: Recording,
    eti: String,
}

impl MbClippyLint for DashETILint {
    fn get_name() -> &'static str {
        "dash_eti"
    }

    async fn check(
        _conn: &mut sqlx::SqliteConnection,
        entity: &musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(None);
        };

        // Check if the title is suspiciously like a remix or vip
        let regex = Regex::new(
            r"(?mi)/.+ - (original mix|sped up|slowed down|extra slowed down|(.+remix))$",
        )
        .unwrap();

        let eti = match regex.captures(&recording.title) {
            None => return Ok(None),
            Some(val) => val
                .get(0)
                .expect("The regex should return this capture group"),
        };

        Ok(Some(Self {
            recording: recording.clone(),
            eti: eti.as_str().to_string(),
        }))
    }

    async fn get_body(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" seems to have ETI after a dash (-). This is often seen on spotify imports
-> Convert the dash to parenthesis: `{} ({})`",
            self.recording
                .pretty_format_with_credits(conn, false)
                .await?,
                self.recording.title.clone().split(" - ").next().unwrap_or(""),
                self.eti
        ))
    }

    async fn get_links(
        &self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Recording".to_string(),
            url: format!("https://musicbrainz.org/recording/{}", self.recording.mbid),
        });

        out.push(MbClippyLintLink {
            name: "Recording editing".to_string(),
            url: format!(
                "https://musicbrainz.org/recording/{}/edit",
                self.recording.mbid
            ),
        });

        Ok(out)
    }

    async fn get_hints(
        &self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintHint>, crate::Error> {
        Ok(vec![MbClippyLintHint::new("\"ETI\" means \"Extra title information\" and refers to information in the title of a track that isn't the name".to_string())])
    }

    fn get_severity(&self) -> crate::models::clippy::lint_severity::LintSeverity {
        LintSeverity::MissingRelation
    }
}
