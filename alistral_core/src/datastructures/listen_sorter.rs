use core::future::Future;

use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::AlistralClient;

/// A specific strategy to add listens to a [`EntityWithListensCollection`]
pub trait ListenSortingStrategy<Ent, Lis>
where
    Ent: HasRowID,
    Lis: ListenCollectionReadable,
{
    fn sort_insert_listen(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Ent, Lis>,
        listen: Listen,
    ) -> impl Future<Output = Result<(), crate::Error>>;

    fn sort_insert_listens(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Ent, Lis>,
        listens: Vec<Listen>,
    ) -> impl Future<Output = Result<(), crate::Error>> {
        async {
            for listen in listens.into_iter() {
                self.sort_insert_listen(client, data, listen).await?;
            }

            Ok(())
        }
    }
}
