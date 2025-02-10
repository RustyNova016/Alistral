use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use sqlx::QueryBuilder;
use sqlx::Sqlite;

use crate::api::listenbrainz::listens::fetch_latest_listens_of_user;
use crate::AlistralClient;

pub struct ListenQuery {
    user: User,
    min_ts: i64,
    max_ts: i64,
    status: Option<ListenStatus>,
    fetch_lb: bool,
}

impl ListenQuery {
    fn create_query(&self) -> QueryBuilder<'_, Sqlite> {
        let mut query: QueryBuilder<'_, Sqlite> =
            QueryBuilder::new("SELECT listens.* FROM listens WHERE 1 = 1 ");

        match self.status {
            Some(ListenStatus::Mapped) => {
                query.push("AND listens.recording_msid IN (SELECT msid_mapping.recording_msid FROM msid_mapping WHERE user = ").push_bind(self.user.id).push(")");
            }
            Some(ListenStatus::Unmapped) => {
                query.push("AND listens.recording_msid NOT IN (SELECT msid_mapping.recording_msid FROM msid_mapping WHERE user = ").push_bind(self.user.id).push(")");
            }
            None => {}
        };

        query.push("AND listened_at >= ").push_bind(self.min_ts);
        query.push("AND listened_at <= ").push_bind(self.max_ts);

        query
    }

    pub async fn fetch_all(
        &self,
        client: &AlistralClient,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Listen>, crate::Error> {
        if self.fetch_lb {
            fetch_latest_listens_of_user(client, conn, &self.user.name).await?;
        }

        let mut create_query = self.create_query();
        let query = create_query.build_query_as::<Listen>();
        Ok(query.fetch_all(conn).await?)
    }
}

pub enum ListenStatus {
    Mapped,
    Unmapped,
}

