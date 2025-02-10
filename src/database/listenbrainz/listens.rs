use alistral_core::api::listenbrainz::listens::fetch_latest_listens_of_user;
use alistral_core::datastructures::listen_collection::ListenCollection;
use macon::Builder;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use sqlx::SqliteConnection;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::utils::env::in_offline_mode;

// #[derive(Builder)]
// pub struct ListenFetchQuery {
//     #[builder(Default=!)]
//     user: String,

//     fetch_recordings_redirects: bool,

//     returns: ListenFetchQueryReturn,
// }

// impl ListenFetchQuery {
//     pub async fn fetch(
//         self,
//         conn: &mut sqlx::SqliteConnection,
//     ) -> Result<ListenCollection, crate::Error> {
//         // Fetch the latest listens
//         // ... If it's not in offline mode
//         if !in_offline_mode() {
//             fetch_latest_listens_of_user(&ALISTRAL_CLIENT, conn, &self.user).await?;
//         }

//         if self.fetch_recordings_redirects {
//             Self::fetch_recordings_redirects(conn, &self.user).await?;
//         }

//         match self.returns {
//             ListenFetchQueryReturn::Mapped => Ok(ListenCollection::new(
//                 Listen::get_mapped_listen_of_user(conn, &self.user).await?,
//             )),
//             ListenFetchQueryReturn::Unmapped => Ok(ListenCollection::new(
//                 Listen::get_unmapped_listen_of_user(conn, &self.user).await?,
//             )),
//             ListenFetchQueryReturn::None => Ok(ListenCollection::default()),
//         }
//     }

//     #[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
//     async fn fetch_recordings_redirects(
//         conn: &mut SqliteConnection,
//         user: &str,
//     ) -> Result<(), crate::Error> {
//         let unfetched = Listen::get_unfetched_recordings_of_user(conn, user).await?;
//         pg_counted!(unfetched.len(), "Fetching listen MBIDs");

//         for id in unfetched {
//             Recording::get_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db, &id).await?;
//             pg_inc!();
//         }

//         Ok(())
//     }
// }

// pub enum ListenFetchQueryReturn {
//     Mapped,
//     Unmapped,
//     None,
// }
