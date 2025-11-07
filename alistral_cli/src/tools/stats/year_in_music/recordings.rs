use std::fmt::Write;

use itertools::Itertools;

use crate::datastructures::cli_formating::title::Heading1;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn recording_report(&self) -> String {
        let mut out = String::new();
        let stats = self.current.recording_stats().await.unwrap();
        let stats = stats.iter().cloned().collect_vec();

        writeln!(out, "{}", Heading1("Best recordings of the year 🏆")).unwrap();
        writeln!(out, "Here's the top 10 tracks of this year:").unwrap();
        writeln!(out, "{}", Self::top_recordings(stats).await).unwrap();

        out
    }

    // async fn new_releases_top(&self, listen_by_dates: &ListenByReleaseDate) {
    //     let rankings = Ranking::from(listen_by_dates.mapping.as_hash_map().values().collect_vec());
    //     let rankings = rankings.get_ranks(|(date)| date);

    //     let rows = rankings.into_iter().map(|(rank, (date, recs))| {

    //     });
    // }


}
