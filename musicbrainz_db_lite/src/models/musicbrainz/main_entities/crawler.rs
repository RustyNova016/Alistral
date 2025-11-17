use core::future::ready;
use std::sync::Arc;

use futures::FutureExt;
use futures::SinkExt as _;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::channel::mpsc::Sender;
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::mpsc::channel;
use futures::channel::mpsc::unbounded;
use futures::stream;
use futures::stream::select_all;
use streamies::Streamies as _;
use tracing::debug;

use crate::DBClient;
use crate::models::musicbrainz::main_entities::MainEntity;

pub fn crawler(
    client: Arc<DBClient>,
    start_nodes: Vec<Arc<MainEntity>>,
) -> impl Stream<Item = Result<Arc<MainEntity>, crate::Error>> {
    // Declare the crawlers
    let (out_sender, out_reciever) = channel(10);
    let (crawl_sender, crawl_reciever) = unbounded();
    #[cfg(feature = "channels-console")]
    let (crawl_sender, crawl_reciever) =
        channels_console::instrument!((crawl_sender, crawl_reciever), log = true);

    let task = crawl_task(out_sender, crawl_reciever, client)
        .into_stream()
        .filter_map(|val| match val {
            Ok(_) => ready(None),
            Err(e) => ready(Some(Err::<Arc<MainEntity>, crate::Error>(e))),
        });

    // Starting nodes of the stream. Those are chained to the reciever stream
    let start_stream = stream::iter(start_nodes);

    let receiver_stream = start_stream
        .chain(out_reciever)
        // Only allow unique entities.
        .unique_by(|item| item.get_unique_id())
        // To make sure that the items passing through are crawled.
        .map(move |item| {
            let mut crawl_sender = crawl_sender.clone();

            async move {
                crawl_sender.send(item.clone()).await?;

                Ok::<Arc<MainEntity>, crate::Error>(item)
            }
        })
        .buffer_unordered(8);

    select_all([receiver_stream.boxed_local(), task.boxed_local()])
}

async fn crawl_task(
    out_sender: Sender<Arc<MainEntity>>,
    crawl_receiver: UnboundedReceiver<Arc<MainEntity>>,
    client: Arc<DBClient>,
) -> Result<(), crate::Error> {
    let mut stream = crawl_receiver
        .unique_by(|item| item.get_unique_id())
        .map(|item| {
            let mut out_sender = out_sender.clone();
            let client = client.clone();

            async move {
                // First send to the consumer
                match out_sender.send(item.clone()).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e.into());
                    }
                }

                debug!("Crawling over entity: {}", item.get_unique_id());

                match &*item {
                    MainEntity::Artist(val) => val.get_crawler(client.clone(), out_sender).await,
                    MainEntity::Recording(val) => val.get_crawler(client.clone(), out_sender).await,
                    MainEntity::Release(val) => val.get_crawler(client.clone(), out_sender).await,
                    MainEntity::Track(val) => val.get_crawler(client.clone(), out_sender).await,

                    _ => Ok(()),
                }
            }
        })
        .buffered(4);

    while (stream.try_next().await?).is_some() {}

    // Close the channel as we don't have anything else to send
    out_sender.clone().close_channel();

    Ok(())
}
