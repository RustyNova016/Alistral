use std::collections::HashSet;

use async_fn_stream::fn_stream;
use async_fn_stream::try_fn_stream;
use futures::pin_mut;
use futures::Stream;
use futures::StreamExt;
use musicbrainz_rs_nova::entity::recording::Recording as MSRecording;
use musicbrainz_rs_nova::Browse;

use crate::api::SaveToDatabase;
use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::recording::Recording;
use crate::DBClient;

impl Artist {
    /// Fetch all the artist's recordings into a stream. it returns a stream of tuple containing (Recording, Total Recordings)
    pub fn fetch_artist_recordings<'conn, 'client>(
        &self,
        conn: &'conn mut sqlx::SqliteConnection,
        client: &'client DBClient,
    ) -> impl Stream<Item = Result<(Recording, i32), crate::Error>> + use<'_, 'conn, 'client> {
        try_fn_stream(|emitter| async move {
            let mut progress = 0;
            let mut offset = 0;
            let mut total = 1;

            while (progress as i32) < total {
                let results = MSRecording::browse()
                    .by_artist(&self.mbid)
                    .with_artist_credits()
                    .with_annotation()
                    .with_genres()
                    .with_isrcs()
                    .with_tags()
                    .with_area_relations()
                    .with_artist_relations()
                    .with_event_relations()
                    .with_genre_relations()
                    .with_instrument_relations()
                    .with_label_relations()
                    .with_place_relations()
                    .with_recording_relations()
                    .with_release_group_relations()
                    .with_release_relations()
                    .with_series_relations()
                    .with_url_relations()
                    .with_work_relations()
                    .limit(90)
                    .offset(offset as u16)
                    .execute_with_client(&client.musicbrainz_client)
                    .await?;

                progress += results.entities.len();
                offset += 90;
                total = results.count;

                for result in results.entities {
                    let data = result.save(conn).await?;

                    emitter.emit((data, total)).await
                }
            }

            Ok(())
        })
    }

    /// Browse the entities in the cache as a stream
    pub fn browse_artist_recordings<'this, 'conn>(
        &'this self,
        conn: &'conn mut sqlx::SqliteConnection,
    ) -> impl Stream<Item = Result<Recording, sqlx::Error>> + Send + use<'this, 'conn> {
        // Wrap the stream into an fn_stream to deal with lifetime issues.
        fn_stream(|emitter| async move {
            let mut stream = sqlx::query_as!(
                Recording,
                "
                SELECT
                    recordings.*
                FROM
                    recordings
                    INNER JOIN artist_credits ON recordings.artist_credit = artist_credits.id
                    INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit
                WHERE
                    artist_credits_item.artist_gid IN (
                        SELECT
                            gid
                        FROM
                            artists_gid_redirect
                        WHERE
                            artists_gid_redirect.new_id = ?
                    )
                ",
                self.id
            )
            .fetch(conn);

            while let Some(data) = stream.next().await {
                emitter.emit(data).await;
            }
        })
    }

    /// Browse the entities in the cache then fetch the entities from MB. All the entities are deduplicated by their MBID
    ///
    /// ⚠️ This stream may return entities that are cached but removed in MB.
    ///
    /// ⚠️ It may also return stale data from the cache even if it updates it later
    pub fn browse_or_fetch_artist_recordings<'this, 'conn, 'client>(
        &'this self,
        conn: &'conn mut sqlx::SqliteConnection,
        client: &'client DBClient,
    ) -> impl Stream<Item = Result<Recording, crate::Error>> + use<'this, 'conn, 'client> {
        try_fn_stream(|emitter| async move {
            let mut unique = HashSet::new();
            // First, browse the cache
            {
                let stream = self.browse_artist_recordings(conn);
                pin_mut!(stream); // Pin the stream for iteration

                while let Some(data) = stream.next().await {
                    let data = data?;
                    unique.insert(data.mbid.clone());

                    emitter.emit(data).await;
                }
            } // We use a scope to drop any shadowed `stream` variable that is caused by `pin_mut!`. This allows us to gain back our `&mut conn`

            // We browsed the cache. Now let's browse from MB
            let mb_stream = self.fetch_artist_recordings(conn, client);
            pin_mut!(mb_stream);

            while let Some(data) = mb_stream.next().await {
                let (data, count) = data?;

                // Check the count. Is the total equal to the already visited entities? If so we visited them all!
                if unique.len() >= count as usize {
                    return Ok(());
                }

                // If there's still unvisited entities, check if this one is visited
                if unique.contains(&data.mbid) {
                    continue;
                }

                // We have an unvisited entity!
                unique.insert(data.mbid.clone());

                emitter.emit(data).await;
            }

            Ok(())
        })
    }
}
