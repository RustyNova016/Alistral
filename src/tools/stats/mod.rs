pub mod target_entity;
use core::fmt::Display;
use std::path::PathBuf;

use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::FromListenCollection;
use alistral_core::datastructures::entity_with_listens::work::WorkWithListens;
use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use clap::Parser;
use clap::ValueEnum;
use derive_more::IsVariant;

use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;
use musicbrainz_db_lite::models::musicbrainz::work::Work;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::artist_stats;
use crate::database::interfaces::statistics_data::recording_stats;
use crate::database::interfaces::statistics_data::recording_strategy;
use crate::database::interfaces::statistics_data::release_group_stats;
use crate::database::interfaces::statistics_data::release_stats;
use crate::database::interfaces::statistics_data::release_strategy;
use crate::database::interfaces::statistics_data::work_stats;
use crate::datastructures::statistic_formater::ListenCountStats;
use crate::datastructures::statistic_formater::ListenDurationStats;
use crate::datastructures::statistic_formater::StatFormatterVariant;
use crate::datastructures::statistic_formater::StatisticFormater;
use crate::datastructures::statistic_formater::StatisticOutput;
use crate::datastructures::statistic_formater::StatisticType;
use crate::models::cli::common::SortSorterBy;
use crate::models::config::Config;

mod artists;
mod recordings;
mod release_groups;
mod releases;
mod work;

#[derive(Parser, Debug, Clone)]
pub struct StatsCommand {
    /// The type of entity to sort by.
    target: StatsTarget,

    /// Name of the user to fetch stats listen from
    #[arg(short, long)]
    username: Option<String>,

    /// The type of sorting to use
    #[arg(short, long, default_value_t = SortBy::ListenCount)]
    sort_by: SortBy,

    /// Recursively add parent works to work stats
    #[arg(long)]
    w_recursive: bool,
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
}

impl StatsTarget {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Recording => "recording",
            Self::Artist => "artist",
            Self::Release => "release",
            Self::ReleaseGroup => "release_group",
            Self::Work => "work",
        }
    }
}

impl StatsCommand {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let user = Config::check_username(&self.username).to_lowercase();

        self.route_sort_type(user).await?;

        Ok(())
    }

    async fn route_sort_type(&self, user: String) -> Result<(), crate::Error> {
        match (self.sort_by, self.target) {
            (SortBy::ListenCount, StatsTarget::Artist) => {
                let data = artist_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<Artist, ListenCountStats>(data).await
            }
            (SortBy::ListenCount, StatsTarget::Recording) => {
                let data = recording_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<Recording, ListenCountStats>(data).await
            }
            (SortBy::ListenCount, StatsTarget::Release) => {
                let data = release_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<Release, ListenCountStats>(data).await
            }
            (SortBy::ListenCount, StatsTarget::ReleaseGroup) => {
                let data = release_group_stats(&ALISTRAL_CLIENT, user.clone()).await?;
                self.run_stats::<ReleaseGroup, ListenCountStats>(data).await
            }
            (SortBy::ListenCount, StatsTarget::Work) => {
                let data = self.work_stats(user).await?;
                self.run_stats::<Work, ListenCountStats>(data).await
            }
            _ => {
                println!(
                    "This type of statistic is not implemented for this entity. If you believe it should be able to exist, feel free to create an issue"
                );
                Ok(())
            }
        }
    }

    async fn run_stats<Ent, S>(
        &self,
        data: EntityWithListensCollection<Ent, ListenCollection>,
    ) -> Result<(), crate::Error>
    where
        Ent: RowId,
        S: StatisticType,
        StatisticFormater<Ent, S>: StatFormatterVariant<Ent>,
    {
        let stats = StatisticFormater::<Ent, S>::new(data, StatisticOutput::Print);

        stats.print_paged().await
    }

    async fn work_stats(&self, user: String) -> Result<WorkWithListensCollection, crate::Error> {
        Ok(ListenFetchQuery::get_entity_with_listens(
            &ALISTRAL_CLIENT.core,
            user,
            &self.work_strategy(),
        )
        .await?)
    }

    fn work_strategy(&self) -> WorkWithListenStrategy {
        let mut strat = WorkWithListenStrategy::new(
            &ALISTRAL_CLIENT.core,
            recording_strategy(&ALISTRAL_CLIENT),
        );

        if self.w_recursive {
            strat = strat.with_recursive_parents()
        }

        strat
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
