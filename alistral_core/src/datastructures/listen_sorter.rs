use core::future::Future;

use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::RowId;

use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;

/// A specific strategy to add listens to a [`EntityWithListensCollection`]
pub trait ListenSortingStrategy<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    fn sort_insert_listen(
        &self,
        data: &mut EntityWithListensCollection<Ent, Lis>,
        listen: Listen,
    ) -> impl Future<Output = Result<(), crate::Error>>;

    fn sort_insert_listens(
        &self,
        data: &mut EntityWithListensCollection<Ent, Lis>,
        listens: Vec<Listen>,
    ) -> impl Future<Output = Result<(), crate::Error>> {
        async {
            for listen in listens.into_iter() {
                self.sort_insert_listen(data, listen).await?;
            }

            Ok(())
        }
    }
}
