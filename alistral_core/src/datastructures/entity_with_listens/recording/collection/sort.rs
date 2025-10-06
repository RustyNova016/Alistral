use std::backtrace::Backtrace;

use itertools::Itertools as _;
use musicbrainz_db_lite::GetConnectionError;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::listen::relations::recording::ListenRecordingDBRel;
use snafu::ResultExt;
use snafu::Snafu;
use tracing::instrument;
use tuillez::pg_spinner;
use tuillez::tracing_utils::pg_future::PGFuture as _;

use crate::AlistralClient;
use crate::database::fetching::recordings::prefetch_recordings_of_listens;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::traits::sorter::Linker;

impl Linker<RecordingWithListensCollection, Listen> for AlistralClient {
    type Returned = Result<(), RecordingStatsError>;

    async fn link_entry(
        &self,
        data: &mut RecordingWithListensCollection,
        item: Listen,
    ) -> Self::Returned {
        Self::link_entry_batch(&self, data, vec![item]).await
    }

    #[instrument(skip(self, data, items), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn link_entry_batch(
        &self,
        data: &mut RecordingWithListensCollection,
        items: Vec<Listen>,
    ) -> Self::Returned {
        // If empty, early return
        if items.is_empty() {
            return Ok(());
        }

        pg_spinner!("Compiling recording listens data");
        let conn = &mut *self
            .musicbrainz_db
            .get_conn()
            .await
            .context(ConnectionSnafu)?;

        // Prefetch data
        prefetch_recordings_of_listens(conn, self, &items)
            .await
            .context(DatabaseSnafu)?;

        let listen_refs = items.iter().collect_vec();

        let joins = Listen::get_related_entity_bulk::<ListenRecordingDBRel>(conn, &listen_refs)
            .pg_spinner("Loading recordings from cache...")
            .await
            .context(DatabaseSnafu)?;

        for (recording, listens) in joins.into_many_to_zero(items).invert() {
            // Insert the listens into the data structure
            if let Some(recording) = recording {
                data.insert_or_merge_listens(recording, listens);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Snafu)]
pub enum RecordingStatsError {
    #[snafu(display("Could not get a connection for the database"))]
    ConnectionError {
        #[snafu(backtrace)]
        source: GetConnectionError,
    },

    #[snafu(display("Something went wrong with the database"))]
    DatabaseError {
        backtrace: Backtrace,
        source: musicbrainz_db_lite::Error,
    },
}
