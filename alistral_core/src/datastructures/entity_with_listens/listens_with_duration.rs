use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::models::listen_with_time::iterator::IntoListenWithDurationIterator;

impl<Ent, Lis> IntoListenWithDurationIterator for EntityWithListens<Ent, Lis>
where
    Lis: IntoListenWithDurationIterator,
{
    fn into_listen_with_duration_iterator(
        self,
    ) -> impl Iterator<Item = crate::models::listen_with_time::ListenWithDuration> {
        self.listens.into_listen_with_duration_iterator()
    }
}
