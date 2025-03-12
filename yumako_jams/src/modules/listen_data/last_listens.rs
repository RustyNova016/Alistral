use std::sync::Arc;

use futures::StreamExt;
use futures::TryStreamExt;
use musicbrainz_db_lite::models::listenbrainz::listen::views::latest_listens::LatestRecordingListensView;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::listen_data::ListenAction;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct LatestListens {
    user: String,
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
            .map_ok(move |mut track| {
                let this = this_moved.clone();
                async move {
                    let conn = &mut *client.get_db_lite_raw_conn().await?;

                    let query = LatestRecordingListensView {
                        user: this.user.clone(),
                        recording: track.recording().mbid.clone(),
                        max_ts: None,
                    };

                    track.set_listens(query.execute(conn).await?, this.action);

                    Ok(track)
                }
            })
            .try_buffered(this.buffer)
            .boxed())
    }
}

fn default_action() -> ListenAction {
    ListenAction::Add
}

fn default_buffer() -> usize {
    8
}
