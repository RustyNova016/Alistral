use futures::StreamExt;
use futures::TryStreamExt;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::listen_data::ListenAction;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClearListens {}

impl RadioModule for ClearListens {
    fn create_stream<'a>(
        self,
        stream: RadioStream<'a>,
        _client: &'a YumakoClient,
    ) -> LayerResult<'a> {
        Ok(stream
            .map_ok(|mut track| {
                track.set_listens(Default::default(), ListenAction::Replace);
                track
            })
            .boxed())
    }
}
