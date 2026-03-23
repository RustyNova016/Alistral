use std::collections::HashMap;
use std::sync::Arc;

use alistral_core::datastructures::listen_collection::ListenCollection;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::stream;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::views::latest_listens::LatestRecordingListensView;
use serde::Deserialize;
use serde::Serialize;
use streamies::TryStreamies;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::listen_data::ListenAction;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestListens {
    user: String,
    #[serde(default = "default_count")]
    count: i64,
    #[serde(default = "default_action")]
    action: ListenAction,
    #[serde(default = "default_buffer")]
    buffer: usize,
}

impl RadioModule for LatestListens {
    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        let this = Arc::new(self);
        let this_moved = this.clone();

        Ok(stream
            .ready_chunks_ok(50000)
            .map_ok(move |tracks| convert_batch(this_moved.clone(), client, tracks))
            .extract_future_ok()
            .buffered(this.buffer)
            .map(|item| match item {
                //TODO: Add merge_results in streamies
                Ok(Err(err)) | Err(err) => Err(err),
                Ok(Ok(val)) => Ok(val),
            })
            .map_ok(stream::iter)
            .flatten_ok()
            .boxed())
    }
}

fn default_count() -> i64 {
    3
}

fn default_action() -> ListenAction {
    ListenAction::Add
}

fn default_buffer() -> usize {
    8
}

async fn convert_batch(
    this: Arc<LatestListens>,
    client: &YumakoClient,
    mut tracks: Vec<RadioItem>,
) -> Result<Vec<RadioItem>, crate::Error> {
    let conn = &mut *client.get_db_lite_raw_conn().await?;

    let recording_ids = tracks
        .iter()
        .map(|t| t.recording().mbid.clone())
        .collect_vec();

    let query = LatestRecordingListensView {
        user: this.user.clone(),
        count: this.count,
        recordings: recording_ids,
        max_ts: None,
    };

    let joins = query.execute(conn).await?;

    let mut map: HashMap<i64, ListenCollection> = HashMap::new();

    for item in joins {
        map.entry(item.original_id)
            .or_default()
            .push_unique(item.data);
    }

    for (id, listens) in map {
        let Some(track) = tracks.iter_mut().find(|t| t.recording().id == id) else {
            continue;
        };

        track.set_listens(listens.into(), this.action);
    }

    Ok(tracks)
}
