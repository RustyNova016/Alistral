use alistral_core::cli::progress_bar::global_progress_bar::PG_FETCHING;
use alistral_core::datastructures::listen_collection::ListenCollection;
use chrono::{DateTime, Utc};
use macon::Builder;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use sqlx::SqliteConnection;

use crate::api::listenbrainz::LISTENBRAINZ_CLIENT;
use crate::utils::env::in_offline_mode;
use crate::utils::println_lis;

/// Fetch the latest listens for the provided user. If the user has no listens, it will do a full listen fetch.
pub async fn fetch_latest_listens_of_user(
    conn: &mut sqlx::SqliteConnection,
    user: &str,
) -> Result<(), musicbrainz_db_lite::Error> {
    let latest_listen_ts = Listen::get_latest_listen_of_user(&mut *conn, user)
        .await?
        .map(|v| v.listened_at);
    let mut pull_ts = Some(Utc::now().timestamp());

    // This loop has two possible states.
    // - Fresh dump:
    //     `latest_listen_ts` is none. We loop until `save_listen_payload_in_transaction` tell us it's over
    //
    // - Incremental dump:
    //     `latest_listen_ts` is set. We loop until pull_ts is before the latest listen
    while (latest_listen_ts.is_none() && pull_ts.is_some())
        || (latest_listen_ts.is_some_and(|a| pull_ts.is_some_and(|b| a <= b)))
    {
        println_lis(format!(
            "Getting listens from before: {} ({})",
            DateTime::from_timestamp(pull_ts.unwrap(), 0).unwrap(),
            pull_ts.unwrap(),
        ));
        pull_ts = Listen::execute_listen_fetch(conn, &LISTENBRAINZ_CLIENT, user, pull_ts.unwrap())
            .await?;
    }

    Ok(())
}

#[derive(Builder)]
pub struct ListenFetchQuery {
    #[builder(Default=!)]
    user: String,

    fetch_recordings_redirects: bool,

    returns: ListenFetchQueryReturn,
}

impl ListenFetchQuery {
    pub async fn fetch(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<ListenCollection, crate::Error> {
        // Fetch the latest listens
        // ... If it's not in offline mode
        if !in_offline_mode() {
            fetch_latest_listens_of_user(conn, &self.user).await?;
        }

        if self.fetch_recordings_redirects {
            Self::fetch_recordings_redirects(conn, &self.user).await?;
        }

        match self.returns {
            ListenFetchQueryReturn::Mapped => Ok(ListenCollection::new(
                Listen::get_mapped_listen_of_user(conn, &self.user).await?,
            )),
            ListenFetchQueryReturn::Unmapped => Ok(ListenCollection::new(
                Listen::get_unmapped_listen_of_user(conn, &self.user).await?,
            )),
            ListenFetchQueryReturn::None => Ok(ListenCollection::default()),
        }
    }

    async fn fetch_recordings_redirects(
        conn: &mut SqliteConnection,
        user: &str,
    ) -> Result<(), crate::Error> {
        let unfetched = Listen::get_unfetched_recordings_of_user(conn, user).await?;
        let subm = PG_FETCHING.get_submitter(unfetched.len() as u64);

        for id in unfetched {
            Recording::get_or_fetch(conn, &id).await?;
            subm.inc(1);
        }

        Ok(())
    }
}

pub enum ListenFetchQueryReturn {
    Mapped,
    Unmapped,
    None,
}
