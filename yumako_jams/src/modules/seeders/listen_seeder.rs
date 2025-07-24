use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;

use crate::RadioStream;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::radio_item::RadioItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct ListenSeeder {
    user: String,
}

impl RadioModule for ListenSeeder {
    fn create_stream<'a>(
        self,
        _stream: RadioStream<'a>,
        client: &'a crate::client::YumakoClient,
    ) -> LayerResult<'a> {
        Ok(try_fn_stream(async |emitter| {
            let tracks = ListenFetchQuery::get_recordings_with_listens(
                &client.alistral_core,
                self.user,
                &RecordingWithListenStrategy::new(&client.alistral_core)
            )
            .await?
            .into_iter();

            for track in tracks {
                emitter.emit(RadioItem::from(track)).await;
            }

            Ok(())
        })
        .boxed())
    }
}
