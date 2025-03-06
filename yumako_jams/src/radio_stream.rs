use futures::StreamExt;
use futures::TryStreamExt as _;
use rust_decimal::Decimal;

use crate::aliases::RadioStream;
use crate::modules::scores::ScoreMerging;
use crate::radio_item::RadioItem;

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
}
