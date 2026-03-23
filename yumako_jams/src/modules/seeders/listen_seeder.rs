use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::traits::sorter::InsertElement;
use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
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
        let user = self.user.clone();
        Ok(try_fn_stream(async move |emitter| {
            // Get the listens
            let tracks = Listen::get_or_fetch_listens()
                .client(&client.alistral_core.musicbrainz_db)
                .incremental(true)
                .users(&[&user])
                .mapped(true)
                .unmapped(true)
                .call()
                .await?;

            // Compile the stats
            let mut coll = RecordingWithListensCollection::new();
            coll.insert_elements(&*client.alistral_core, tracks).await?;

            // Emit the results
            for track in coll {
                emitter.emit(RadioItem::from(track)).await;
            }

            Ok(())
        })
        .boxed())
    }
}
