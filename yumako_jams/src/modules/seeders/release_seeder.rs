use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use futures::stream::select;
use musicbrainz_db_lite::GetOrFetch;
use musicbrainz_db_lite::Release;
use serde::Deserialize;
use serde::Serialize;
use snafu::OptionExt;
use snafu::ResultExt;


use crate::modules::error::ReleaseSeederSnafu;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReleaseSeeder {
    release_mbids: Vec<String>,
}

impl RadioModule for ReleaseSeeder {
    fn create_stream<'a>(
        self,
        mut stream: crate::RadioStream<'a>,
        client: &'a crate::YumakoClient,
    ) -> crate::modules::radio_module::LayerResult<'a> {
        for release in self.release_mbids {
            stream = select(stream, create_artist_stream(client, release)?).boxed()
        }

        Ok(stream.boxed())
    }
}

fn create_artist_stream(
    client: &crate::YumakoClient,
    release_mbid: String,
) -> crate::modules::radio_module::LayerResult<'_> {
    Ok(try_fn_stream(async move |emitter| {
        // Load the artist
        let release = Release::get_or_fetch_as_task(
            client.alistral_core.musicbrainz_db.clone(),
            &release_mbid,
        )
        .await
        .context(DatabaseSnafu {
            mbid: release_mbid.clone(),
        })
        .context(ReleaseSeederSnafu)?
        .context(MissingReleaseSnafu {
            mbid: release_mbid.clone(),
        })
        .context(ReleaseSeederSnafu)?;

        let conn = &mut client.get_db_lite_raw_conn().await?;

        let recordings = release
            .get_recordings_or_fetch(conn, &client.alistral_core.musicbrainz_db)
            .await
            .context(DatabaseSnafu {
                mbid: release_mbid.clone(),
            })
            .context(ReleaseSeederSnafu)?;

        for recording in recordings {
            emitter.emit(RadioItem::from(recording)).await;
        }

        Ok(())
    })
    .boxed())
}

#[derive(Debug, snafu::Snafu)]
pub enum ReleaseSeederError {
    #[snafu(display("A database error happened while getting the artist with mbid {mbid}"))]
    DatabaseError {
        source: musicbrainz_db_lite::Error,

        mbid: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    #[snafu(display("Couldn't find the artist with mbid {mbid}"))]
    MissingRelease {
        mbid: String,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
