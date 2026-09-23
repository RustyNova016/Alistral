use async_fn_stream::TryStreamEmitter;
use async_fn_stream::fn_stream;
use futures::StreamExt;
use futures::TryStreamExt as _;
use futures::stream::BoxStream;
use rust_decimal::Decimal;

use crate::models::radio_stream::radio_item::RadioItem;
use crate::modules::scores::ScoreMerging;

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

    /// Combination of `map_ok` and `set_score` on the [`RadioItem`]s.
    fn map_scores<F>(self, f: F, merge: ScoreMerging, layer_id: String) -> RadioStream<'a>
    where
        F: Fn(&RadioItem) -> Decimal + Send + 'a,
    {
        self.map_ok(move |mut t| {
            let score = f(&t);
            t.update_score(score, merge, &layer_id);
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


    // fn try_filter_items<Fut, F>(self, f: F, layer_id: String)
    // where
    //     Fut: Future<Output = bool>,
    //     F: FnMut(&Self::Ok) -> Fut,
    //     Self: Sized,
    // {
    //     self.try_filter(|item| {
    //         if f(item) {
    //             trace!("[{layer_id}] Keeping {}", item.entity().get_mbid())
    //             true
    //         } else {
    //             trace!("[{layer_id}] Removing {}", item.entity().get_mbid())
    //             true
    //         }
    //     })
    // }
}