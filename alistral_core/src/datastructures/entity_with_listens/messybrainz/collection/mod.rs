use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;

use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::messybrainz::MessybrainzWithListens;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use crate::datastructures::listen_collection::ListenCollection;

pub type MessybrainzWithListensCollection =
    EntityWithListensCollection<MessybrainzSubmission, ListenCollection>;

impl MessybrainzWithListensCollection {
    pub async fn from_listencollection_default(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        let listens = listens.iter_listens().collect_vec();
        let results = Listen::get_messybrainz_data_from_listen_as_batch(conn, &listens).await?;

        // Convert
        let mut out = Self::new();

        for (_, (listen, messybrainz_datas)) in results {
            for messybrainz_data in messybrainz_datas {
                let mut listens = ListenCollection::default();
                listens.push(listen.to_owned().to_owned());

                out.insert_or_merge_entity_stats(MessybrainzWithListens {
                    entity: messybrainz_data,
                    listens,
                });
            }
        }

        Ok(out)
    }
}
