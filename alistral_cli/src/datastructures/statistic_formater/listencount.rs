use std::sync::LazyLock;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsync;

use crate::datastructures::statistic_formater::ListenCountStats;
use crate::datastructures::statistic_formater::StatFormatterVariant;
use crate::datastructures::statistic_formater::StatisticFormater;
use crate::utils::constants::LISTENBRAINZ_FMT;

/// The formatter for the listen count statistics
pub struct ListenCountFormatter<'l> {
    pub mb_format: &'l MusicbrainzFormater,
}

pub static LISTENCOUNT_FMT: LazyLock<ListenCountFormatter> =
    LazyLock::new(|| ListenCountFormatter {
        mb_format: &LISTENBRAINZ_FMT,
    });

// === Implement the line formatter for statistics holders ===

impl<'l, Ent, Lis> FormatWithAsync<ListenCountFormatter<'l>> for EntityWithListens<Ent, Lis>
where
    Ent: HasRowID + FormatWithAsync<MusicbrainzFormater> + Sync,
    Lis: ListenCollectionReadable + Sync,
    crate::Error: From<<Ent as FormatWithAsync<MusicbrainzFormater>>::Error>,
{
    type Error = crate::Error;

    async fn format_with_async(
        &self,
        ft: &ListenCountFormatter<'l>,
    ) -> Result<String, Self::Error> {
        Ok(format!(
            "[{}] {}",
            self.listen_count(),
            self.entity().format_with_async(ft.mb_format).await?
        ))
    }
}

// === Implement the formatter for the whole statistic collection ===

impl<Ent, Lis> StatFormatterVariant<Ent, Lis> for StatisticFormater<Ent, Lis, ListenCountStats>
where
    Ent: HasRowID + for<'a> FormatWithAsync<MusicbrainzFormater> + Sync,
    Lis: ListenCollectionReadable + Sync,
    crate::Error: for<'a> From<<Ent as FormatWithAsync<MusicbrainzFormater>>::Error>,
{
    fn sort_elements(&self) -> Vec<&EntityWithListens<Ent, Lis>> {
        let mut data = self.data.iter().collect_vec();
        data.sort_by_cached_key(|e| std::cmp::Reverse(e.listen_count()));
        data
    }

    async fn get_line(
        &self,
        element: &EntityWithListens<Ent, Lis>,
    ) -> Result<String, crate::Error> {
        element.format_with_async(&*LISTENCOUNT_FMT).await
    }
}
