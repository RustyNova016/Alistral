use std::fmt::Write as _;

use color_eyre::owo_colors::OwoColorize;
use extend::ext;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::artist_credit::ArtistCredits;
use musicbrainz_db_lite::models::musicbrainz::label::Label;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::api::clients::ALISTRAL_CLIENT;

use super::hyperlink_rename;

#[ext]
pub impl MainEntity {
    async fn pretty_format(
        &self,
        conn: &mut sqlx::SqliteConnection,
        listenbrainz: bool,
    ) -> Result<String, crate::Error> {
        let out = match self {
            MainEntity::Artist(val) => val.pretty_format(listenbrainz).await?,
            MainEntity::Label(val) => val.pretty_format().await?,
            MainEntity::Recording(val) => {
                val.pretty_format_with_credits(conn, listenbrainz).await?
            }
            MainEntity::Release(val) => val.pretty_format_with_credits(conn, listenbrainz).await?,
            MainEntity::Work(val) => val.pretty_format().await?,
        };

        Ok(out)
    }
}

fn format_disambiguation(title: &str, disambiguation: &Option<String>) -> String {
    let dis = match disambiguation {
        None => "",
        Some(val) => {
            if !val.is_empty() {
                &format!(" ({})", &val).truecolor(175, 175, 175).to_string()
            } else {
                ""
            }
        }
    };

    format!("{title}{dis}")
}

#[ext]
pub impl Artist {
    async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.name.truecolor(20, 163, 249).to_string(),
                &Some(self.disambiguation.clone()),
            ),
            &self.get_url_link(listenbrainz),
        ))
    }

    fn get_url_link(&self, listenbrainz: bool) -> String {
        if !listenbrainz {
            format!("https://musicbrainz.org/artist/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/artist/{}", &self.mbid)
        }
    }
}

#[ext]
pub impl ArtistCredits {
    async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        let mut out = String::new();

        for credit in &self.1 {
            let link = if !listenbrainz {
                format!("https://musicbrainz.org/artist/{}", &credit.artist_gid)
            } else {
                format!("https://listenbrainz.org/artist/{}", &credit.artist_gid)
            };

            write!(
                out,
                "{}{}",
                hyperlink_rename(&credit.name.truecolor(20, 163, 249), &link),
                credit.join_phrase
            )
            .expect("Display format is infaillible");
        }
        Ok(out)
    }
}

#[ext]
pub impl Label {
    async fn pretty_format(&self) -> Result<String, crate::Error> {
        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.name.truecolor(214, 0, 214).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/label/{}", &self.mbid),
        ))
    }
}

#[ext]
pub impl Recording {
    async fn pretty_format(&self) -> Result<String, crate::Error> {
        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(0, 214, 114).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/recording/{}", &self.mbid),
        ))
    }

    async fn pretty_format_with_credits(
        &self,
        conn: &mut sqlx::SqliteConnection,
        listenbrainz: bool,
    ) -> Result<String, crate::Error> {
        Ok(format!(
            "{} by {}",
            self.pretty_format().await?,
            self.get_artist_credits_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?
                .pretty_format(listenbrainz)
                .await?
        ))
    }
}

#[ext]
pub impl Release {
    async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(242, 244, 123).to_string(),
                &self.disambiguation,
            ),
            &self.get_url_link(listenbrainz),
        ))
    }

    async fn pretty_format_with_credits(
        &self,
        conn: &mut sqlx::SqliteConnection,
        listenbrainz: bool,
    ) -> Result<String, crate::Error> {
        Ok(format!(
            "{} by {}",
            self.pretty_format(listenbrainz).await?,
            self.get_artist_credits_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?
                .pretty_format(listenbrainz)
                .await?
        ))
    }

    fn get_url_link(&self, listenbrainz: bool) -> String {
        if !listenbrainz {
            format!("https://musicbrainz.org/release/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/release/{}", &self.mbid)
        }
    }
}

#[ext]
pub impl Work {
    async fn pretty_format(&self) -> Result<String, crate::Error> {
        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(0, 214, 214).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/work/{}", &self.mbid),
        ))
    }
}

#[ext]
pub impl ReleaseGroup {
    async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(254, 173, 75).to_string(),
                &Some(self.disambiguation.clone()),
            ),
            &self.get_url_link(listenbrainz),
        ))
    }

    async fn pretty_format_with_credits(
        &self,
        conn: &mut sqlx::SqliteConnection,
        listenbrainz: bool,
    ) -> Result<String, crate::Error> {
        Ok(format!(
            "{} by {}",
            self.pretty_format(listenbrainz).await?,
            self.get_artist_credits_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?
                .pretty_format(listenbrainz)
                .await?
        ))
    }

    fn get_url_link(&self, listenbrainz: bool) -> String {
        if !listenbrainz {
            format!("https://musicbrainz.org/release-group/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/album/{}", &self.mbid)
        }
    }
}
