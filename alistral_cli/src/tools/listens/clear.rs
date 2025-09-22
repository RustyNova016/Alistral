use clap::Parser;

use crate::ALISTRAL_CLIENT;

/// Remove all the listens in the database.
///
/// Optionally only target one user
#[derive(Parser, Debug, Clone)]
pub struct ListensClearCommand {
    user: Option<String>,
}

impl ListensClearCommand {
    pub async fn run(&self) {
        let conn = &mut *ALISTRAL_CLIENT
            .musicbrainz_db
            .get_raw_connection()
            .await
            .expect("Couldn't get a connection");

        match &self.user {
            Some(user) => {
                sqlx::query!("DELETE FROM listens WHERE LOWER(user) = LOWER(?)", user)
                    .execute(conn)
                    .await
                    .expect("Couldn't delete listens");
            }
            None => {
                sqlx::query!("DELETE FROM listens")
                    .execute(conn)
                    .await
                    .expect("Couldn't delete listens");
            }
        }
    }
}
