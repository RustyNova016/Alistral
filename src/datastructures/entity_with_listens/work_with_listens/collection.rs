use alistral_core::cli::progress_bar::global_progress_bar::PG_FETCHING;
use alistral_core::datastructures::listen_collection::ListenCollection;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::datastructures::entity_with_listens::entity_with_listen_collection::EntityWithListensCollection;

pub type WorkWithListensCollection = EntityWithListensCollection<Work, ListenCollection>;

impl WorkWithListensCollection {
    pub async fn add_parents_recursive(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), crate::Error> {
        let mut queue = self.0.values().cloned().collect_vec();
        let mut seen = Vec::new();
        let mut count = queue.len() as u64;

        let progress_bar = PG_FETCHING.get_submitter(queue.len() as u64);
        while let Some(work) = queue.pop() {
            if seen.contains(&work.work().mbid.clone()) {
                continue;
            }
            let new_works = work.get_parents(conn).await?;

            for new_work in new_works {
                queue.push(new_work.clone());
                self.insert_or_merge(new_work);
                count += 1;
            }
            progress_bar.inc(1);
            progress_bar.set_count(count);
            seen.push(work.work().mbid.clone());
        }

        Ok(())
    }
}
