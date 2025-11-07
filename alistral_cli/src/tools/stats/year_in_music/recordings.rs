use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn recording_report(&self) -> String {
        self.new_release_page().await
    }

    // async fn new_releases_top(&self, listen_by_dates: &ListenByReleaseDate) {
    //     let rankings = Ranking::from(listen_by_dates.mapping.as_hash_map().values().collect_vec());
    //     let rankings = rankings.get_ranks(|(date)| date);

    //     let rows = rankings.into_iter().map(|(rank, (date, recs))| {

    //     });
    // }
}
