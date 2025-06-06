use async_fn_stream::try_fn_stream;
use futures::Stream;
use futures::TryStreamExt;
use futures::pin_mut;
use interzic::models::playlist_stub::PlaylistStub;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rand::prelude::SliceRandom;
use rand::rng;
use tracing::info;
use tracing::warn;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::radio::collector::RadioCollector;
use crate::datastructures::radio::seeders::listens::ListenSeeder;
use crate::models::cli::radio::RadioExportTarget;
use crate::models::data_storage::DataStorage;
use crate::models::error::ResultTEExt as _;
use crate::tools::radio::convert_recordings;
use crate::utils::constants::LISTENBRAINZ_FMT;
use crate::utils::data_file::DataFile as _;

pub async fn create_radio_mix(
    conn: &mut sqlx::SqliteConnection,
    seeder: ListenSeeder,
    token: String,
    unlistened: bool,
    collector: RadioCollector,
    target: RadioExportTarget,
) {
    let username = seeder.username().clone();

    info!("[Seeding] Getting listens");
    let recordings_with_listens = seeder
        .seed(conn)
        .await
        .expect_fatal("Couldn't find seed listens");

    let recordings = recordings_with_listens.iter_entities().collect_vec();

    let radio = RadioCircle::new(unlistened);

    let collected = {
        let radio_stream = radio.into_stream(conn, recordings);

        info!("[Finalising] Creating radio playlist");
        pin_mut!(radio_stream);

        collector
            .try_collect(radio_stream)
            .await
            .expect_fatal("Error while generating the playlist")
    };

    let counter = DataStorage::load().expect_fatal("Couldn't load data storage");
    let playlist = PlaylistStub {
        title: format!(
            "Radio: Circles #{}",
            counter.write().unwrap().incr_playlist_count()
        ),
        description: "Automatically generated by: https://github.com/RustyNova016/Alistral"
            .to_string(),
        recordings: convert_recordings(conn, collected)
            .await
            .expect_fatal("Couldn't convert recordings for playlist"),
    };

    target
        .export(playlist, Some(username), Some(&token))
        .await
        .expect_fatal("Couldn't send the playlist");
}

#[derive(Debug)]
pub struct RadioCircle {
    unlistened: bool,
    artist_blacklist: Vec<String>,
    recording_blacklist: Vec<String>,
}

impl RadioCircle {
    pub fn new(unlistened: bool) -> Self {
        Self {
            unlistened,
            ..Default::default()
        }
    }

    async fn get_random_recording_from_artist(
        &self,
        conn: &mut sqlx::SqliteConnection,
        artist: &Artist,
    ) -> Result<Option<Recording>, crate::Error> {
        info!(
            "Checking artist: {}",
            artist.format_with_async(&LISTENBRAINZ_FMT).await?
        );
        let mut recordings: Vec<Recording> = artist
            .browse_or_fetch_artist_recordings(conn, ALISTRAL_CLIENT.musicbrainz_db.clone())
            .try_collect()
            .await?;

        let mut rng = rng();
        recordings.shuffle(&mut rng);

        for recording in recordings {
            if self.recording_blacklist.contains(&recording.mbid) {
                continue;
            }

            return Ok(Some(recording));
        }

        Ok(None)
    }

    async fn get_random_artist_from_recordings(
        &self,
        conn: &mut sqlx::SqliteConnection,
        mut recordings: Vec<&Recording>,
    ) -> Result<Option<Artist>, crate::Error> {
        let mut rng = rng();
        recordings.shuffle(&mut rng);

        for recording in recordings {
            let mut artists = recording
                .get_artists_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?;

            artists.shuffle(&mut rng);

            for artist in artists {
                if self.artist_blacklist.contains(&artist.mbid) {
                    continue;
                }

                return Ok(Some(artist));
            }
        }

        Ok(None)
    }

    /// Get an item of the playlist
    async fn get_random_item(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        recordings: Vec<&Recording>,
    ) -> Result<Option<Recording>, crate::Error> {
        if self.unlistened {
            recordings
                .iter()
                .for_each(|r| self.recording_blacklist.push(r.mbid.clone()));
        }

        loop {
            let artist = self
                .get_random_artist_from_recordings(conn, recordings.clone())
                .await?;

            match artist {
                Some(artist) => {
                    let recording = self.get_random_recording_from_artist(conn, &artist).await?;

                    match recording {
                        Some(recording) => {
                            self.recording_blacklist.push(recording.mbid.clone());
                            return Ok(Some(recording));
                        }
                        None => {
                            warn!(
                                "{} has not enough recordings for generation. Consider adding more recordings to Musicbrainz!",
                                artist.name
                            );
                            self.artist_blacklist.push(artist.mbid.clone());
                        }
                    }
                }
                None => return Ok(None),
            }
        }
    }

    pub fn into_stream<'conn, 'recordings>(
        mut self,
        conn: &'conn mut sqlx::SqliteConnection,
        recordings: Vec<&'recordings Recording>,
    ) -> impl Stream<Item = Result<Recording, crate::Error>> + use<'conn, 'recordings> {
        try_fn_stream(|emitter| async move {
            while let Some(val) = self.get_random_item(conn, recordings.clone()).await? {
                emitter.emit(val).await;
            }

            Ok(())
        })
    }
}
impl Default for RadioCircle {
    fn default() -> Self {
        Self {
            unlistened: false,
            artist_blacklist: vec![
                "125ec42a-7229-4250-afc5-e057484327fe".to_string(), // Ignore [unknown]
                "89ad4ac3-39f7-470e-963a-56509c546377".to_string(), // Ignore Verious Artist
            ],
            recording_blacklist: Vec::new(),
        }
    }
}
