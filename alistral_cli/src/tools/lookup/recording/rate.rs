use core::fmt::Write as _;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use tuillez::extensions::chrono_exts::DateTimeUtcExt as _;
use tuillez::extensions::chrono_exts::DurationExt as _;

use crate::tools::lookup::lookup_components::comp_arrow::LookupCompArrow;
use crate::tools::lookup::lookup_components::local_date::LookupLocalDate;
use crate::tools::lookup::recording::printer::RecordingLookup;

impl RecordingLookup {
    pub async fn get_listen_rate_section(&self) -> String {
        let mut section = String::new();

        writeln!(&mut section, "[Listen Rate]").unwrap();
        writeln!(
            &mut section,
            "   - {}",
            self.get_dur_between_listens_field().await
        )
        .unwrap();
        writeln!(
            &mut section,
            "   - {}",
            self.get_estimated_date_of_next_listen_field().await
        )
        .unwrap();

        section
    }

    async fn get_dur_between_listens_field(&self) -> String {
        let now_data = self
            .get_now_target_recording_stats()
            .await
            .average_duration_between_listens()
            .floor_to_minute();
        let mut string = format!(
            "Average duration between listens: {}",
            now_data.to_humantime().unwrap()
        );

        if let Some(before_data) = self
            .get_before_target_recording_stats()
            .await
            .map(|rec| rec.average_duration_between_listens().floor_to_minute())
            .filter(|dur| !dur.is_zero())
        {
            string = format!(
                "{string} [{} {}]",
                LookupCompArrow::comp_desc(now_data, before_data),
                before_data.to_humantime().unwrap()
            );
        }

        string
    }

    async fn get_estimated_date_of_next_listen_field(&self) -> String {
        let date = self
            .get_all_time_target_recording_stats()
            .await
            .listens()
            .estimated_date_of_next_listen()
            .map(|d| d.floor_to_second());

        format!("Estimated next listen: {}", LookupLocalDate(date))
    }
}
