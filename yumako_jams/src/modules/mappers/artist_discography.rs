use async_fn_stream::try_fn_stream;
use futures::StreamExt as _;
use futures::TryStreamExt;
use futures::pin_mut;
use futures::stream;
use futures::stream::BoxStream;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use serde::Deserialize;
use streamies::TryStreamies as _;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;

#[derive(Debug, Deserialize)]
pub struct ArtistDiscographyMapper {}

impl RadioModule for ArtistDiscographyMapper {
    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        Ok(stream
            .map_ok(|radio_item| radio_item.entity().clone())
            .unique_by_ok(|r| r.id)
            .map_ok(|rec| get_artist_streams_from_recording(client, rec))
            .extract_future_ok()
            .buffer_unordered(8)
            .flatten_result_ok()
            .try_flatten_unordered(8) //TODO: Make it a variable
            .unique_by_ok(|r| r.id)
            .map_ok(RadioItem::from)
            .boxed())
    }
}

async fn get_artist_streams_from_recording(
    client: &YumakoClient,
    recording: Recording,
) -> Result<BoxStream<'_, Result<Recording, crate::Error>>, crate::Error> {
    let conn = &mut client.get_db_lite_raw_conn().await?;

    let artists = recording
        .get_artists_or_fetch(conn, &client.alistral_core.musicbrainz_db)
        .await?;

    Ok(stream::iter(artists)
        .flat_map_unordered(None, |art| get_stream_from_artist(client, art))
        .boxed())
}

fn get_stream_from_artist(
    client: &YumakoClient,
    artist: Artist,
) -> BoxStream<'_, Result<Recording, crate::Error>> {
    try_fn_stream(|emit| async move {
        let conn = &mut client.get_db_lite_raw_conn().await?;
        let stream = artist.browse_or_fetch_artist_recordings(conn, client.alistral_core.musicbrainz_db.clone());

        pin_mut!(stream);
        while let Some(item) = stream.next().await {
            match item {
                Ok(v) => emit.emit(v).await,
                Err(e) => emit.emit_err(e.into()).await,
            };
        }

        Ok(())
    })
    .boxed()
}
