use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::pin_mut;
use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::GetOrFetch;
use musicbrainz_db_lite::HasMBID;
use serde::Deserialize;
use serde::Serialize;
use snafu::OptionExt;
use snafu::ResultExt;
use tracing::trace;

use crate::RadioStream;
use crate::YumakoClient;
use crate::models::radio_stream::radio_item::RadioItem;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::error::ArtistSeederSnafu;
use crate::modules::radio_module::LayerResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct ArtistSeeder {
    artist_mbids: Vec<String>,
}

impl RadioModule<ArtistSeeder> {
    /// Add the module to the stream
    pub fn into_stream<'a>(
        self,
        mut stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        for artist in self.inputs.artist_mbids {
            let moved_id = self.id.clone();

            let stream2 = create_artist_stream(client, artist)?.inspect_ok(move |item| {
                trace!(
                    "[{}] Seeded recording {}",
                    moved_id.clone(),
                    item.entity().get_mbid()
                );
            });

            stream = stream.chain(stream2).boxed()
        }

        Ok(stream.boxed())
    }
}

fn create_artist_stream(client: &crate::YumakoClient, artist_mbid: String) -> LayerResult<'_> {
    Ok(try_fn_stream(async move |emitter| {
        // Load the artist
        let artist =
            Artist::get_or_fetch_as_task(client.alistral_core.musicbrainz_db.clone(), &artist_mbid)
                .await
                .context(DatabaseSnafu {
                    mbid: artist_mbid.clone(),
                })
                .context(ArtistSeederSnafu)?
                .context(MissingArtistSnafu {
                    mbid: artist_mbid.clone(),
                })
                .context(ArtistSeederSnafu)?;

        let conn = &mut client.get_db_lite_raw_conn().await?;

        let stream = artist
            .browse_or_fetch_artist_recordings(conn, client.alistral_core.musicbrainz_db.clone());

        pin_mut!(stream);
        while let Some(item) = stream.next().await {
            match item {
                Ok(v) => emitter.emit(RadioItem::from(v)).await,
                Err(e) => emitter.emit_err(e.into()).await,
            };
        }

        Ok(())
    })
    .boxed())
}

#[derive(Debug, snafu::Snafu)]
pub enum ArtistSeederError {
    #[snafu(display("A database error happened while getting the artist with mbid {mbid}"))]
    DatabaseError {
        source: musicbrainz_db_lite::Error,

        mbid: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(display("Couldn't find the artist with mbid {mbid}"))]
    MissingArtist {
        mbid: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
