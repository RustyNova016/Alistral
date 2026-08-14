use crate::ALISTRAL_CLIENT;

/// Cleanup the cache's data by removing data not closely associated to listens
#[derive(clap::Parser, Debug, Clone)]
pub struct CacheCleanCommand {}

impl CacheCleanCommand {
    pub async fn run(&self) {
        let mut conn = ALISTRAL_CLIENT.get_conn().await;
        let mut count;
        let mut last_count = 0;
        loop {
            sqlx::query("
            DELETE FROM recordings WHERE recordings.id IN (
                SELECT DISTINCT recordings.id
                FROM recordings
                    INNER JOIN recordings_gid_redirect ON recordings_gid_redirect.new_id = recordings.id
                WHERE recordings_gid_redirect.gid NOT IN (
                    SELECT recording_mbid FROM msid_mapping
                ) 
                LIMIT 100
            )
            ").execute(&mut *conn).await.unwrap();

            count = sqlx::query_scalar("SELECT COUNT(*) FROM recordings")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
            println!("Recordings remaining: {count}");

            if count == last_count {
                break;
            }

            last_count = count;
        }

        println!("Done!");
    }
}
