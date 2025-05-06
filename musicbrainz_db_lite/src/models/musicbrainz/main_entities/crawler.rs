use core::future::ready;
use std::sync::Arc;

use futures::FutureExt;
use futures::SinkExt as _;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::channel::mpsc::Sender;
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::mpsc::UnboundedSender;
use futures::channel::mpsc::channel;
use futures::channel::mpsc::unbounded;
use futures::stream::select;
use streamies::Streamies as _;

use crate::DBClient;
use crate::models::musicbrainz::main_entities::MainEntity;

pub fn crawler(
    client: Arc<DBClient>,
    first_item: Arc<MainEntity>,
) -> impl Stream<Item = Result<Arc<MainEntity>, crate::Error>> {
    let (out_sender, out_reciever) = channel(10);
    let (crawl_sender, crawl_reciever) = unbounded();

    let task = crawl_task(
        out_sender,
        crawl_sender.clone(),
        crawl_reciever,
        client,
        first_item,
    )
    .into_stream()
    .filter_map(|val| match val {
        Ok(_) => ready(None),
        Err(e) => ready(Some(Err::<Arc<MainEntity>, crate::Error>(e))),
    });

    let receiver_stream = out_reciever
        .unique_by(|item| item.get_unique_id())
        .map(move |item| {
            let mut crawl_sender = crawl_sender.clone();

            async move {
                crawl_sender.send(item.clone()).await?;

                Ok::<Arc<MainEntity>, crate::Error>(item)
            }
        })
        .buffer_unordered(8);

    select(receiver_stream, task)
}

async fn crawl_task(
    out_sender: Sender<Arc<MainEntity>>,
    crawl_sender: UnboundedSender<Arc<MainEntity>>,
    crawl_receiver: UnboundedReceiver<Arc<MainEntity>>,
    client: Arc<DBClient>,
    first_item: Arc<MainEntity>,
) -> Result<(), crate::Error> {
    crawl_sender.clone().send(first_item).await?;

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

                match &*item {
                    MainEntity::Artist(val) => val.get_crawler(client.clone(), out_sender).await,
                    MainEntity::Recording(val) => val.get_crawler(client.clone(), out_sender).await,
                    MainEntity::Release(val) => val.get_crawler(client.clone(), out_sender).await,
                    MainEntity::Work(val) => val.get_crawler(client.clone(), out_sender).await,
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
