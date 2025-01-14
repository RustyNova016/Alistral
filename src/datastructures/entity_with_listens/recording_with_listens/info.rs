use chrono::Duration;
use chrono::Local;
use color_eyre::owo_colors::OwoColorize;
use humantime::format_duration;
use indoc::formatdoc;
use rust_decimal::Decimal;

use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::models::config::Config;
use crate::utils::extensions::chrono_ext::DateTimeUtcExt;
use crate::utils::extensions::chrono_ext::DurationExt;

use super::collection::RecordingWithListensCollection;
use super::RecordingWithListens;

impl RecordingWithListens {
    /// Generate a formated string with all the informations to display in reports
    pub async fn get_lookup_report(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other_listens: &RecordingWithListensCollection,
    ) -> Result<String, crate::Error> {
        if self.is_empty() {
            return self.generate_empty_report(conn).await;
        }

        self.generate_full_report(conn, other_listens).await
    }

    async fn generate_empty_report(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<String, crate::Error> {
        Ok(format!(
            "{}
                    
        The recording hasn't been listened to yet",
            self.get_title(conn).await?,
        ))
    }

    async fn generate_full_report(
        &self,
        conn: &mut sqlx::SqliteConnection,
        other_listens: &RecordingWithListensCollection,
    ) -> Result<String, crate::Error> {
        let conf = Config::load_or_panic();
        let global_count = self.get_global_listen_count().await?;

        let text = formatdoc! {"
            {title}
            
            [General]
               - Rank: #{rank}
               - Listen count: {listen_count} ({global_count} worldwide)
               - Total playtime: {minu_listened} minutes ({hours_listened} hours)
               - First listened on: {first_listened}
               - Last listened on: {last_listened}
        
            [Listening rate]
               - Average duration between listens: {average_dur}
               - Estimated date of next listen: {next_listen_date}
               {overdue}

            [Radios]
               - Underrated score: {underrated_score}
               - Overdue score: {overdue_score}
               - Overdue score (with multiplier): {overdue_mul}
            ",
            title = self.get_title(conn).await?,
            rank = other_listens.get_rank(&self.recording().mbid).expect("The recording should be listened"),
            listen_count = self.listen_count(),
            minu_listened = self.get_time_listened().map(|dur| dur.deci_minutes().trunc_with_scale(2)).unwrap_or(Decimal::ZERO), // TODO: Proper error
            hours_listened = self.get_time_listened().map(|dur| dur.format_hh_mm()).unwrap_or_else(|| "??".to_string()), // TODO: Proper error
            first_listened = self.first_listen_date().unwrap().floor_to_second().with_timezone(&Local),
            last_listened = self.last_listen_date().unwrap().floor_to_second().with_timezone(&Local),
            average_dur = format_duration(
                self
                    .average_duration_between_listens()
                    .floor_to_minute()
                    .to_std()
                    .unwrap()
            ),
            next_listen_date =         self
            .estimated_date_of_next_listen()
            .unwrap()
            .floor_to_second(),
            overdue = get_overdue_line(self),
            underrated_score = self.get_underated_score(other_listens, global_count).trunc_with_scale(2),
            overdue_score = self.overdue_factor().trunc_with_scale(2)  + Decimal::ONE,
            overdue_mul = ((self.overdue_factor() + Decimal::ONE)
            * conf.read_or_panic().bumps.get_multiplier(&self.recording().mbid.clone()))
        .trunc_with_scale(2)
        };

        Ok(text)
    }

    async fn get_title(&self, conn: &mut sqlx::SqliteConnection) -> Result<String, crate::Error> {
        let raw = format!(
            "\n Statistics of {} ",
            self.recording().format_with_credits(conn).await?
        );
        Ok(format!("{}", raw.on_green().black().bold()))
    }
}

fn get_overdue_line(recording_info: &RecordingWithListens) -> String {
    let time = recording_info.overdue_by();

    if time <= Duration::zero() {
        return String::new();
    }

    format!(
        "- Overdue by: {}",
        format_duration(time.floor_to_minute().to_std().unwrap())
    )
}
