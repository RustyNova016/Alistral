use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::GetOrFetch;
use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::Release;
use tuillez::formatter::FormatWithAsync as _;
use tuillez::utils::hyperlink_rename;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::clippy::lints::sambl::SamblLint;
use crate::sambl::api_results::AlbumData;
use crate::utils::formater;

pub struct MissingSamblReleaseLint {
    release_name: String,
    spotify_link: String,

    artist: Artist,

    potential_release: Option<Release>,
}

impl MissingSamblReleaseLint {
    // pub async fn get_all_lints<'a>(
    //     client: &'a SymphonyzeClient,
    //     entity: &'a MainEntity,
    // ) -> Result<BoxStream<'a, Result<(AlbumData, Option<Self>), crate::Error>>, crate::Error> {
    //     let MainEntity::Artist(artist) = entity else {
    //         return Ok(stream::empty().boxed());
    //     };

    //     let host = Host::Domain("open.spotify.com");
    //     let Some(spot_url) = artist.has_url_with_host(&client.mb_database, &host).await? else {
    //         return Ok(stream::empty().boxed());
    //     };
    //     let spot_id = spot_url
    //         .ressource
    //         .split("/")
    //         .last()
    //         .expect("Malformed spotify url");

    //     let Some(albums) = sambl_get_artist_albums(spot_id, &artist.mbid).await? else {
    //         return Ok(stream::empty().boxed());
    //     };

    //     Ok(stream::iter(albums.album_data)
    //         .map(async move |album| {
    //             let artist = artist.clone();
    //             Self::check_lint(client, &artist, &album)
    //                 .await
    //                 .map(|lint| (album, lint))
    //         })
    //         .buffered(1)
    //         .boxed())
    // }
}

impl SamblLint for MissingSamblReleaseLint {
    async fn check_album_data(
        client: &SymphonyzeClient,
        artist: &Artist,
        sambl_album: &AlbumData,
    ) -> Result<Option<Self>, crate::Error> {
        if sambl_album.album_status == "green" {
            return Ok(None);
        }

        // Sambl found a missing album. Let's make sure it is not present in MB
        // by checking if a release contain the spotify url
        let releases = Release::get_or_fetch_by_url_as_task(
            client.mb_database.clone(),
            &sambl_album.url,
        )
        .await?;

        if !releases.is_empty() {
            // The releases are properly linked. We ignore this report
            return Ok(None);
        }

        // If it's orange, seek the release
        let potential_release = if !sambl_album.mbid.as_ref().is_none_or(|id| id.is_empty()) {
            Release::get_or_fetch_as_task(client.mb_database.clone(), &sambl_album.mbid.clone().unwrap())
                .await?
        } else {
            None
        };

        Ok(Some(Self {
            release_name: sambl_album.name.clone(),
            spotify_link: sambl_album.url.clone(),
            artist: artist.clone(),
            potential_release,
        }))
    }
}

impl MbClippyLint for MissingSamblReleaseLint {
    fn get_name() -> &'static str {
        "missing_sambl_release_lint"
    }

    async fn check(
        _client: &SymphonyzeClient,
        _entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        unimplemented!("Use check_lint instead")
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "SAMBL found a release not in Musicbrainz. Artist {} has `{}` which is not found in MB",
            self.artist.format_with_async(&formater(client)).await?,
            self.release_name
        ))
    }

    async fn get_links(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Spotify Release".to_string(),
            url: self.spotify_link.clone(),
        });

        out.push(MbClippyLintLink {
            name: "Harmony search".to_string(),
            url: format!(
                "https://harmony.pulsewidth.org.uk/release?url={}&category=preferred",
                self.spotify_link
            ),
        });

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let mut ret = Vec::new();

        if let Some(release) = &self.potential_release {
            let link = hyperlink_rename(
                &"release".to_string(),
                &format!("https://musicbrainz.org/release/{}", release.mbid),
            );
            ret.push(MbClippyLintHint::new(format!(
                "This {link} might be the same, but is missing its spotify link."
            )));
        }

        Ok(ret)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingData
    }
}
