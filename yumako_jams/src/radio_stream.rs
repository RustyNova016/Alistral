use async_fn_stream::TryStreamEmitter;
use async_fn_stream::fn_stream;
use chrono::Duration;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryStreamExt as _;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use rust_decimal::Decimal;

use crate::modules::scores::ScoreMerging;
use crate::radio_item::RadioItem;

/// The stream output of the radio
pub type RadioStream<'a> = BoxStream<'a, RadioResult>;

/// Whever that radio item has enountered an error or not
pub type RadioResult = Result<RadioItem, crate::Error>;

/// A radio stream without errors
pub type RadioItemStream<'a> = BoxStream<'a, RadioItem>;

#[extend::ext]
pub impl<'a> RadioStream<'a> {
    fn set_scores<F>(self, f: F, merge: ScoreMerging) -> RadioStream<'a>
    where
        F: Fn(&RadioItem) -> Decimal + Send + 'a,
    {
        self.map_ok(move |mut t| {
            let score = f(&t);
            t.set_score(score, merge);
            t
        })
        .boxed()
    }

    /// Remove the errors of the stream by reemitting them early
    fn to_item_stream(
        mut self,
        try_emitter: &'a TryStreamEmitter<RadioItem, crate::Error>,
    ) -> RadioItemStream<'a> {
        fn_stream(|emitter| async move {
            while let Some(item) = self.next().await {
                match item {
                    Ok(val) => emitter.emit(val).await,
                    Err(err) => try_emitter.emit_err(err).await,
                }
            }
        })
        .boxed()
    }

    fn collect_with(
        mut self,
        min_count: u64,
        min_duration: Duration,
    ) -> BoxFuture<'a, Vec<RadioResult>> {
        async move {
            let mut out = Vec::new();

            while let Some(track) = self.next().await {
                out.push(track);

                let has_minimum_count = min_count <= out.len() as u64;
                let has_sufficient_duration = out
                    .iter()
                    .map(|r| match r {
                        Ok(r) => r.entity().length_as_duration().unwrap_or_default(),
                        Err(_) => Duration::zero(),
                    })
                    .sum::<Duration>()
                    >= min_duration;

                if has_minimum_count && has_sufficient_duration {
                    return out;
                }
            }

            out
        }
        .boxed()
    }
}
