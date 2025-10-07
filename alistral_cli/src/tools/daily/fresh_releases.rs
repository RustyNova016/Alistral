use core::cmp::Reverse;

use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::TimeZone as _;
use chrono::Utc;
use itertools::Itertools as _;
use musicbrainz_db_lite::GetOrFetch;
use musicbrainz_db_lite::ReleaseGroup;
use tracing::instrument;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync;
use tuillez::pg_spinner;

use crate::ALISTRAL_CLIENT;
use crate::api::listenbrainz::fresh_releases::FreshReleaseRelease;
use crate::api::listenbrainz::fresh_releases::FreshReleaseRequest;
use crate::models::config::Config;
use crate::tools::daily::DailyCommand;
use crate::utils::constants::LISTENBRAINZ_FMT;

impl DailyCommand {
    pub async fn print_fresh_releases(stats: &ListenStatisticsData, today: DateTime<Utc>) {
        let fresh_releases = Self::get_fresh_releases(stats, today).await;
        if fresh_releases.is_empty() {
            return;
        }

        println!(
            "{}",
            " Fresh releases of the past 7 days 🌱 "
                .on_green()
                .black()
                .bold()
        );

        for fresh_release in fresh_releases {
            let rg = ReleaseGroup::get_or_fetch_as_task(
                ALISTRAL_CLIENT.musicbrainz_db.clone(),
                &fresh_release.release_group_mbid,
            )
            .await
            .expect("Couldn't retrieve the release group data")
            .expect("Couldn't find the release group");

            println!(
                "   - {} {}",
                rg.format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .expect("Couldn't get artist credits"),
                format!(
                    "({})",
                    Utc.timestamp_opt(rg.first_release_date.unwrap(), 0)
                        .unwrap()
                        //TODO: #407
                        .format("%d/%m/%Y")
                )
                .truecolor(175, 175, 175)
            );
        }
    }

    #[instrument(skip(stats, today), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn get_fresh_releases(
        stats: &ListenStatisticsData,
        today: DateTime<Utc>,
    ) -> Vec<FreshReleaseRelease> {
        pg_spinner!("Generating `Fresh Releases` report");

        let fresh_releases = FreshReleaseRequest::builder()
            .days(7)
            .future(false)
            .past(true)
            .release_date(today)
            .build()
            .fetch()
            .await
            .expect("Couldn't get fresh listens")
            .payload
            .releases;

        let artist_stats = stats.artists_stats().await.unwrap();

        let min_avg = Config::load_or_panic()
            .read_or_panic()
            .get_artist_listened_to_threshold();
        let listened_artist_ids = artist_stats
            .iter()
            .filter(|artist| artist.average_listen_per_entity_listened() >= min_avg)
            .map(|artist| artist.entity().mbid.clone())
            .collect_vec();

        let mut out = fresh_releases
            .into_iter()
            .filter(|release| {
                release
                    .artist_mbids
                    .iter()
                    .any(|mbid| listened_artist_ids.contains(mbid))
            })
            .collect_vec();

        out.sort_by_cached_key(|r| {
            let date = NaiveDate::parse_from_str(&r.release_date, "%Y-%m-%d")
                .expect("Invalid date returned from Listenbrainz");
            Reverse(date)
        });

        out
    }
}
