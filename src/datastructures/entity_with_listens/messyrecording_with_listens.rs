use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;

use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::datastructures::listen_collection::ListenCollection;
use std::collections::HashMap;

/// Represent a messybrainz recording id
pub struct MessyRecordingWithListens {
    pub messybrainz_data: MessybrainzSubmission,
    pub associated_listens: ListenCollection,
}

impl MessyRecordingWithListens {
    pub async fn from_listencollection<T: ListenCollectionLike>(
        conn: &mut sqlx::SqliteConnection,
        listens: T,
    ) -> Result<HashMap<String, Self>, crate::Error> {
        let listens = listens.iter_listens().collect_vec();
        let results = Listen::get_messybrainz_data_from_listen_as_batch(conn, &listens).await?;

        // Convert
        let mut out = HashMap::new();

        for (_, (listen, messybrainz_datas)) in results {
            for messybrainz_data in messybrainz_datas {
                out.entry(messybrainz_data.msid.clone())
                    .or_insert_with(|| Self {
                        messybrainz_data,
                        associated_listens: ListenCollection::default(),
                    })
                    .associated_listens
                    .push((*listen).to_owned());
            }
        }

        Ok(out)
    }

    pub fn get_latest_listen(&self) -> Option<&Listen> {
        self.associated_listens
            .iter()
            .max_by_key(|listen| listen.listened_at)
    }

    pub fn get_oldest_listen(&self) -> Option<&Listen> {
        self.associated_listens
            .iter()
            .min_by_key(|listen| listen.listened_at)
    }
}

impl ListenCollectionLike for MessyRecordingWithListens {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.associated_listens.iter_listens()
    }
}
