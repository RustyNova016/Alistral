use futures::StreamExt;
use futures::TryStreamExt;
use musicbrainz_db_lite::HasMBID;
use serde::Deserialize;
use serde::Serialize;
use tracing::trace;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::models::radio_stream::radio_module::RadioModule;
use crate::modules::listen_data::ListenAction;
use crate::modules::radio_module::LayerResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClearListens {}

impl RadioModule<ClearListens> {
    pub fn into_stream<'a>(self, stream: RadioStream<'a>, _: &'a YumakoClient) -> LayerResult<'a> {
        let module_id = self.id.clone();

        Ok(stream
            .map_ok(move |mut track| {
                track.set_listens(Default::default(), ListenAction::Replace);
                trace!(
                    "[{}] Clearing the listens of {}, in cooldown",
                    module_id,
                    track.entity().get_mbid()
                );
                track
            })
            .boxed())
    }
}
