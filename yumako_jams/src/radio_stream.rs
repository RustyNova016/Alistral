use async_fn_stream::TryStreamEmitter;
use async_fn_stream::fn_stream;
use chrono::Duration;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryStreamExt as _;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use rust_decimal::Decimal;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::modules::scores::ScoreMerging;
use crate::radio_item::RadioItem;
use crate::radio_variables::RadioVariables;

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
        self,
        min_count: u64,
        min_duration: Duration,
    ) -> BoxFuture<'a, Vec<RadioResult>> {
        collect_with_inner(self, min_count, min_duration).boxed()
    }

    fn collect_with_args(
        self,
        args: RadioVariables,
    ) -> Result<BoxFuture<'a, Vec<RadioResult>>, crate::Error> {
        let min_count = args.get_count().transpose()?.unwrap_or(50);

        let min_duration = args
            .get_duration()
            .transpose()?
            .unwrap_or_else(Duration::zero);

        Ok(collect_with_inner(self, min_count, min_duration).boxed())
    }
}

#[instrument(skip(this), fields(indicatif.pb_show = tracing::field::Empty))]
async fn collect_with_inner(
    mut this: RadioStream<'_>,
    min_count: u64,
    min_duration: Duration,
) -> Vec<RadioResult> {
    let mut out = Vec::new();
    let mut prog = 0;
    pg_counted!(100, "Collecting Radio");

    while let Some(track) = this.next().await {
        out.push(track);

        let collected_duration = out
            .iter()
            .map(|r| match r {
                Ok(r) => r.entity().length_as_duration().unwrap_or_default(),
                Err(_) => Duration::zero(),
            })
            .sum::<Duration>();

        let count_prog = (out.len() as u64 * 100)
            .checked_div(min_count)
            .unwrap_or(100);

        let dur_prog = (collected_duration.num_seconds() * 100)
            .checked_div(min_duration.num_seconds())
            .unwrap_or(1) as u64;

        let tot_prog = count_prog.min(dur_prog);

        if tot_prog > prog {
            pg_inc!(tot_prog - prog);
            prog = tot_prog;
        }

        let has_minimum_count = min_count <= out.len() as u64;
        let has_sufficient_duration = collected_duration >= min_duration;

        if has_minimum_count && has_sufficient_duration {
            return out;
        }
    }

    out
}
