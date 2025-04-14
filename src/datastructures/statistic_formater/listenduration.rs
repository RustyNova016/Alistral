use std::sync::LazyLock;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools as _;
use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::extensions::chrono_exts::DurationExt;
use tuillez::formatter::FormatWithAsync;

use crate::datastructures::statistic_formater::ListenDurationStats;
use crate::datastructures::statistic_formater::StatFormatterVariant;
use crate::datastructures::statistic_formater::StatisticFormater;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub struct ListenDurationFormatter<'l> {
    pub mb_format: &'l MusicbrainzFormater<'l>,
}

pub static LISTENDURATION_FMT: LazyLock<ListenDurationFormatter> =
    LazyLock::new(|| ListenDurationFormatter {
        mb_format: &LISTENBRAINZ_FMT,
    });

// === Implement the line formatter for statistics holders ===

impl<'l, Ent, Lis> FormatWithAsync<ListenDurationFormatter<'l>> for EntityWithListens<Ent, Lis>
where
    Self: ListenCollWithTime,
    Ent: RowId + FormatWithAsync<MusicbrainzFormater<'l>> + Sync,
    Lis: ListenCollectionReadable + Sync,
    crate::Error: From<<Ent as FormatWithAsync<MusicbrainzFormater<'l>>>::Error>,
{
    type Error = crate::Error;

    async fn format_with_async(
        &self,
        ft: &ListenDurationFormatter<'l>,
    ) -> Result<String, Self::Error> {
        Ok(format!(
            "[{}] {}",
            self.get_time_listened()
                .unwrap_or_default()
                .floor_to_minute()
                .to_humantime()
                .unwrap_or_default(),
            self.entity().format_with_async(ft.mb_format).await?
        ))
    }
}

// === Implement the formatter for the whole statistic collection ===

impl<Ent, Lis> StatFormatterVariant<Ent, Lis> for StatisticFormater<Ent, Lis, ListenDurationStats>
where
    Ent: RowId + for<'a> FormatWithAsync<MusicbrainzFormater<'a>> + Sync,
    Lis: ListenCollectionReadable + Sync,
    EntityWithListens<Ent, Lis>: ListenCollWithTime,
    crate::Error: for<'a> From<<Ent as FormatWithAsync<MusicbrainzFormater<'a>>>::Error>,
{
    fn sort_elements(&self) -> Vec<&EntityWithListens<Ent, Lis>> {
        let mut data = self.data.iter().collect_vec();
        data.sort_by_cached_key(|e| std::cmp::Reverse(e.get_time_listened().unwrap_or_default()));
        data
    }

    async fn get_line(
        &self,
        element: &EntityWithListens<Ent, Lis>,
    ) -> Result<String, crate::Error> {
        element.format_with_async(&*LISTENDURATION_FMT).await
    }
}
