use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use futures::pin_mut;
use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::GetOrFetch;
use serde::Deserialize;
use serde::Serialize;
use snafu::OptionExt;
use snafu::ResultExt;

use crate::modules::error::ArtistSeederSnafu;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct ArtistSeeder {
    artist_mbid: String,
}

impl RadioModule for ArtistSeeder {
    fn create_stream<'a>(
        self,
        _stream: crate::RadioStream<'a>,
        client: &'a crate::YumakoClient,
    ) -> crate::modules::radio_module::LayerResult<'a> {
        let artist_mbid = self.artist_mbid.clone();

        Ok(try_fn_stream(async move |emitter| {
            // Load the artist
            let artist = Artist::get_or_fetch_as_task(
                client.alistral_core.musicbrainz_db.clone(),
                &artist_mbid,
            )
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

            let stream = artist.browse_or_fetch_artist_recordings(
                conn,
                client.alistral_core.musicbrainz_db.clone(),
            );

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
