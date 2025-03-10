use async_fn_stream::TryStreamEmitter;
use async_fn_stream::fn_stream;
use futures::StreamExt;
use futures::TryStreamExt as _;
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
}
