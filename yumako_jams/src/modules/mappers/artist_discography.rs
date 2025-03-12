use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use async_fn_stream::try_fn_stream;
use futures::StreamExt as _;
use futures::TryStreamExt as _;
use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rand::rng;
use rand::seq::SliceRandom as _;
use serde::Deserialize;
use streamies::Streamies;
use tracing::info;
use tracing::warn;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;
use crate::radio_stream::RadioStreamaExt;

#[derive(Debug, Deserialize)]
pub struct ArtistDiscographyMapper;

impl RadioModule for ArtistDiscographyMapper {
    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        Ok(try_fn_stream(|emitter| async move {
            let mut data = ListenedArtistData::default();
            let recordings = stream
                .to_item_stream(&emitter)
                .map(|r| r.entity().clone())
                .collect_vec()
                .await;

            while let Some(val) = data
                .get_random_item(client, recordings.iter().collect_vec().clone())
                .await
                .transpose()
            {
                match val {
                    Ok(val) => {
                        let recording_listens = RecordingWithListens::new(val, Vec::new().into());

                        emitter.emit(RadioItem::from(recording_listens)).await;
                    }
                    Err(err) => emitter.emit_err(err).await,
                }
            }

            Ok(())
        })
        .boxed())
    }
}

#[derive(Debug, Default)]
pub struct ListenedArtistData {
    artist_blacklist: Vec<String>,
    recording_blacklist: Vec<String>,
}

impl ListenedArtistData {
    async fn get_random_recording_from_artist(
        &self,
        client: &YumakoClient,
        artist: &Artist,
    ) -> Result<Option<Recording>, crate::Error> {
        info!(
            "Checking artist: {}",
            artist.pretty_format(true).await.unwrap()
        );

        let conn = &mut client.get_db_lite_raw_conn().await?;

        let mut recordings: Vec<Recording> = artist
            .browse_or_fetch_artist_recordings(conn)
            .try_collect()
            .await?;

        recordings.shuffle(&mut rng());

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
        client: &YumakoClient,
        mut recordings: Vec<&Recording>,
    ) -> Result<Option<Artist>, crate::Error> {
        let conn = &mut client.get_db_lite_raw_conn().await?;
        recordings.shuffle(&mut rng());

        for recording in recordings {
            let mut artists = recording
                .get_artists_or_fetch(conn, &client.alistral_core.musicbrainz_db)
                .await?;

            artists.shuffle(&mut rng());

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
        client: &YumakoClient,
        recordings: Vec<&Recording>,
    ) -> Result<Option<Recording>, crate::Error> {
        loop {
            let artist = self
                .get_random_artist_from_recordings(client, recordings.clone())
                .await?;

            match artist {
                Some(artist) => {
                    let recording = self
                        .get_random_recording_from_artist(client, &artist)
                        .await?;

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
}
