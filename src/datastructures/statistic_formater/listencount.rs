use std::sync::LazyLock;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use itertools::Itertools as _;
use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsync;

use crate::datastructures::statistic_formater::ListenCountStats;
use crate::datastructures::statistic_formater::StatFormatterVariant;
use crate::datastructures::statistic_formater::StatisticFormater;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub(super) struct ListenCountFormatter<'l> {
    pub mb_format: &'l MusicbrainzFormater<'l>,
}

pub static LISTENCOUNT_FMT: LazyLock<ListenCountFormatter> =
    LazyLock::new(|| ListenCountFormatter {
        mb_format: &LISTENBRAINZ_FMT,
    });

impl<'l, T, L> FormatWithAsync<ListenCountFormatter<'l>> for EntityWithListens<T, L>
where
    T: RowId + FormatWithAsync<MusicbrainzFormater<'l>> + Sync,
    L: ListenCollectionReadable + Sync,
    crate::Error: From<<T as FormatWithAsync<MusicbrainzFormater<'l>>>::Error>,
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

// === Statistic Formater ===

impl<Ent> StatFormatterVariant<Ent> for StatisticFormater<Ent, ListenCountStats>
where
    Ent: RowId + for<'a> FormatWithAsync<MusicbrainzFormater<'a>> + Sync,
    crate::Error: for<'a> From<<Ent as FormatWithAsync<MusicbrainzFormater<'a>>>::Error>
{
    fn sort_elements(&self) -> Vec<&EntityWithListens<Ent, ListenCollection>> {
        let mut data = self.data.iter().collect_vec();
        data.sort_by_cached_key(|e| std::cmp::Reverse(e.listen_count()));
        data
    }

    async fn get_line(
        &self,
        element: &EntityWithListens<Ent, ListenCollection>,
    ) -> Result<String, crate::Error> {
        element.format_with_async(&LISTENCOUNT_FMT).await
    }
}
