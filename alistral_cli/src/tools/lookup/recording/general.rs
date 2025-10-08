use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

use crate::tools::lookup::lookup_components::comp_arrow::LookupCompArrow;
use crate::tools::lookup::lookup_components::duration_string::LookupDurationString;
use crate::tools::lookup::lookup_components::local_date::LookupLocalDate;
use crate::tools::lookup::recording::printer::RecordingLookup;

impl RecordingLookup {
    pub async fn get_rank_field(&self) -> String {
        let now_stats = self.now_recording_stats().await;
        let rank = now_stats.get_rank(&self.recording).unwrap() + 1;

        let mut string = format!("Rank: #{}", rank);

        if let Some(rank_before) = self
            .before_recording_stats()
            .await
            .and_then(|stats| stats.get_rank(&self.recording))
            .map(|r| r + 1)
        {
            string = format!(
                "{string} [{} {rank_before}]",
                LookupCompArrow::comp_desc(rank, rank_before),
            );
        }

        string
    }

    pub async fn get_listen_count_field(&self) -> String {
        let now_count = self.get_now_target_recording_stats().await.listens().len();
        let mut string = format!("Listen count: {now_count}");

        if let Some(before_count) = self
            .get_before_target_recording_stats()
            .await
            .map(|rec| rec.listens().len())
        {
            string = format!(
                "{string} [{} {before_count}]",
                LookupCompArrow::comp_asc(now_count, before_count),
            );
        }

        string
    }

    pub async fn get_playtime_field(&self) -> String {
        let now_data = self
            .get_now_target_recording_stats()
            .await
            .get_time_listened();

        let mut string = format!("Total playtime: {}", LookupDurationString(now_data));

        if let Some(before_data) = self
            .get_before_target_recording_stats()
            .await
            .map(|rec| rec.get_time_listened())
        {
            string = format!(
                "{string} [{} {}]",
                LookupCompArrow::comp_asc(now_data, before_data),
                LookupDurationString(before_data)
            );
        }

        string
    }

    pub async fn get_first_listen_date_field(&self) -> String {
        let date = self
            .get_all_time_target_recording_stats()
            .await
            .listens()
            .oldest_listen_date();

        format!("First listened on: {}", LookupLocalDate(date))
    }

    pub async fn get_last_listen_date_field(&self) -> String {
        let date = self
            .get_all_time_target_recording_stats()
            .await
            .listens()
            .latest_listen_date();

        format!("Last listened on: {}", LookupLocalDate(date))
    }
}
