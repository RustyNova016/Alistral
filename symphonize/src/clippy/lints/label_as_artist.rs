use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::Tag;
use musicbrainz_db_lite::models::shared_traits::db_relation::ArtistFromCreditsRelation;
use tuillez::formatter::FormatWithAsync;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::utils::formater;

pub struct LabelAsArtistLint {
    entity: MainEntity,
    label_artist: Artist,
}

impl LabelAsArtistLint {
    async fn get_artists(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Vec<Artist>, crate::Error> {
        match entity {
            MainEntity::Recording(val) => Ok(val
                .get_related_entity_or_fetch_as_task::<ArtistFromCreditsRelation>(
                    &client.mb_database,
                )
                .await?),
            MainEntity::Release(val) => Ok(val
                .get_related_entity_or_fetch_as_task::<ArtistFromCreditsRelation>(
                    &client.mb_database,
                )
                .await?),
            MainEntity::Track(val) => Ok(val
                .get_related_entity::<ArtistFromCreditsRelation>(
                    &mut *client
                        .mb_database
                        .clone()
                        .get_raw_connection_as_task()
                        .await?,
                )
                .await?),
            //TODO: Release Group
            _ => Ok(Vec::new()),
        }
    }
}

impl MbClippyLint for LabelAsArtistLint {
    fn get_name() -> &'static str {
        "label_as_artist"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let artists = Self::get_artists(client, entity).await?;
        let conn = &mut client.mb_database.get_raw_connection().await?;

        for artist in artists {
            let tags = Tag::query_from_entity(conn, &artist).await?;

            if tags.iter().any(|tag| tag.name == "label as artist") {
                return Ok(Some(Self {
                    entity: entity.clone(),
                    label_artist: artist,
                }));
            }
        }

        Ok(None)
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "\"{}\" is credited to {}, which is a label set as an artist. It should probably be removed, and properly set as the label of the release",
            self.entity.format_with_async(&formater(client)).await?,
            self.label_artist
                .format_with_async(&formater(client))
                .await?
        ))
    }

    async fn get_links(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Entity:".to_string(),
            url: self.entity.get_musicbrainz_link(),
        });

        out.push(MbClippyLintLink {
            name: "Entity editing".to_string(),
            url: format!("{}/edit", self.entity.get_musicbrainz_link()),
        });

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        Ok(Vec::new())
        //Ok(vec![MbClippyLintHint::new("Digital releases may use label artists to simulate a label page. ".to_string())])
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::WrongData
    }
}
