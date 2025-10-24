use core::fmt::Display;

use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use alistral_core::datastructures::entity_with_listens::tags::id::SimpleTag;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Utc;
use clap::Parser;
use clap::ValueEnum;
use derive_more::IsVariant;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::Label;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::artist_stats;
use crate::database::interfaces::statistics_data::release_group_stats;
use crate::database::interfaces::statistics_data::release_stats;
use crate::datastructures::statistic_formater::ListenCountStats;
use crate::datastructures::statistic_formater::ListenDurationStats;
use crate::datastructures::statistic_formater::StatFormatterVariant;
use crate::datastructures::statistic_formater::StatisticFormater;
use crate::datastructures::statistic_formater::StatisticOutput;
use crate::datastructures::statistic_formater::StatisticType;
use crate::models::cli::common::Timeframe;
use crate::models::datastructures::tops::scorer::listen_count::ListenCountTopScorer;
use crate::models::datastructures::tops::scorer::listen_duration::ListenDurationTopScorer;
use crate::tools::stats::tops::generator::TopGenerator;
use crate::utils::user_inputs::UserInputParser;

pub mod generate_rows;
pub mod generator;
pub mod printing;
pub mod stats_compiling;
pub mod target_entity;

/// Retrieve the top listened entities
#[derive(Parser, Debug, Clone)]
pub struct StatsTopCommand {
    /// The type of entity to sort by.
    target: StatsTarget,

    /// Name of the user to fetch stats listen from
    #[arg(long)]
    username: Option<String>,

    /// The type of sorting to use
    #[arg(short, long, default_value_t = SortBy::ListenCount)]
    sort_by: SortBy,

    /// Recursively add parent works to work stats
    #[arg(long)]
    w_recursive: bool,

    /// Time period to use for the statistics.
    #[clap(short, long)]
    timeframe: Option<Timeframe>,

    /// Get statistics from this date. Use YYYY-MM-DD format
    #[clap(short, long)]
    from: Option<NaiveDate>,

    /// Get statistics until this date. Use YYYY-MM-DD format
    #[clap(short, long)]
    until: Option<NaiveDate>,
}

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum SortBy {
    /// The number of times the entity has been listened to
    ListenCount,

    /// The total duration this entity has been listened for
    ListenDuration,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListenCount => write!(f, "listen-count")?,
            Self::ListenDuration => write!(f, "listen-duration")?,
        };

        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug, Copy, IsVariant)]
pub enum StatsTarget {
    Recording,
    Artist,
    Release,
    ReleaseGroup,
    Work,
    Tag,
    Label,
}

impl Display for StatsTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Self::Recording => "recording",
            Self::Artist => "artist",
            Self::Release => "release",
            Self::ReleaseGroup => "release_group",
            Self::Work => "work",
            Self::Tag => "tag",
            Self::Label => "label",
        };

        write!(f, "{inner}")
    }
}

impl StatsTopCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);

        self.route_sort_type(username).await.unwrap();
    }

    pub fn from(&self) -> Option<DateTime<Utc>> {
        if let Some(t) = &self.timeframe {
            return Some(t.get_start_date());
        }

        if let Some(t) = &self.from {
            //TODO: Proper error?
            return Some(
                t.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .to_utc(),
            );
        }

        None
    }

    pub fn until(&self) -> Option<DateTime<Utc>> {
        if self.timeframe.is_some() {
            return Some(Utc::now());
        }

        if let Some(t) = &self.until {
            //TODO: Proper error?
            return Some(
                t.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .to_utc(),
            );
        }

        None
    }

    pub async fn get_generator(&self) -> TopGenerator {
        let username = UserInputParser::username_or_default(&self.username);
        let stats = ALISTRAL_CLIENT.statistics_of_user(username).await;
        TopGenerator::new(stats, self.from(), self.until())
    }

    async fn route_sort_type(&self, user: String) -> Result<(), crate::Error> {
        match (self.sort_by, self.target) {
            (SortBy::ListenCount, StatsTarget::Artist) => {
                let data = artist_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<Artist, RecordingWithListensCollection, ListenCountStats>(data)
                    .await
            }
            (SortBy::ListenCount, StatsTarget::Recording) => {
                let gene = self.get_generator().await;
                gene.print_recording_stats(ListenCountTopScorer).await;
                Ok(())
            }
            (SortBy::ListenCount, StatsTarget::Release) => {
                let data = release_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<Release, RecordingWithListensCollection, ListenCountStats>(data)
                    .await
            }
            (SortBy::ListenCount, StatsTarget::ReleaseGroup) => {
                let data = release_group_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<ReleaseGroup, ReleaseWithRecordingsCollection, ListenCountStats>(
                    data,
                )
                .await
            }
            (SortBy::ListenCount, StatsTarget::Work) => {
                let data = self.work_stats(user).await?;
                self.run_stats::<Work, RecordingWithListensCollection, ListenCountStats>(data)
                    .await
            }
            (SortBy::ListenCount, StatsTarget::Tag) => {
                let data = self.tag_stats(user).await?;
                self.run_stats::<SimpleTag, RecordingWithListensCollection, ListenCountStats>(data)
                    .await
            }
            (SortBy::ListenCount, StatsTarget::Label) => {
                let data = self.label_stats(user).await?;
                self.run_stats::<Label, ReleaseWithRecordingsCollection, ListenCountStats>(data)
                    .await
            }

            // ====================
            // Listen duration
            // ====================
            (SortBy::ListenDuration, StatsTarget::Artist) => {
                let data = artist_stats(&ALISTRAL_CLIENT, user).await?;

                self.run_stats::<Artist, RecordingWithListensCollection, ListenDurationStats>(data)
                    .await
            }
            (SortBy::ListenDuration, StatsTarget::Recording) => {
                let gene = self.get_generator().await;
                gene.print_recording_stats(ListenDurationTopScorer).await;
                Ok(())
            }
            (SortBy::ListenDuration, StatsTarget::Release) => {
                let data = release_stats(&ALISTRAL_CLIENT, user.clone()).await?;

                self.run_stats::<Release, RecordingWithListensCollection, ListenDurationStats>(data)
                    .await
            }
            (SortBy::ListenDuration, StatsTarget::ReleaseGroup) => {
                let data = release_group_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<ReleaseGroup, ReleaseWithRecordingsCollection, ListenDurationStats>(
                    data,
                )
                .await
            }
            (SortBy::ListenDuration, StatsTarget::Work) => {
                let data = self.work_stats(user).await?;
                self.run_stats::<Work, RecordingWithListensCollection, ListenDurationStats>(data)
                    .await
            }
            (SortBy::ListenDuration, StatsTarget::Tag) => {
                let data = self.tag_stats(user).await?;
                self.run_stats::<SimpleTag, RecordingWithListensCollection, ListenDurationStats>(
                    data,
                )
                .await
            }
            (SortBy::ListenDuration, StatsTarget::Label) => {
                let data = self.label_stats(user).await?;
                self.run_stats::<Label, ReleaseWithRecordingsCollection, ListenDurationStats>(data)
                    .await
            } // _ => {
              //     println!(
              //         "This type of statistic is not implemented for this entity. If you believe it should be able to exist, feel free to create an issue"
              //     );
              //     Ok(())
              // }
        }
    }

    async fn run_stats<Ent, Lis, S>(
        &self,
        data: EntityWithListensCollection<Ent, Lis>,
    ) -> Result<(), crate::Error>
    where
        Ent: HasRowID,
        Lis: ListenCollectionReadable,
        S: StatisticType,
        StatisticFormater<Ent, Lis, S>: StatFormatterVariant<Ent, Lis>,
    {
        let stats = StatisticFormater::<Ent, Lis, S>::new(data, StatisticOutput::Print);

        stats.print_paged().await
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::database::get_conn;
//     use crate::models::cli::common::StatsTarget;
//     use crate::tools::stats::stats_command;
//     use crate::tools::stats::SortSorterBy;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn stats_command_recordings() {
//          let mut conn = get_conn().await;
//         stats_command(&mut conn, "RustyNova", StatsTarget::WorkRecursive, SortSorterBy::Count).await;
//     }
// }

